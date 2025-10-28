use core::{
    cell::{Cell, RefCell},
    ffi::c_void,
    pin::Pin,
    ptr::null_mut,
    sync::atomic::{self, AtomicPtr, AtomicU32, compiler_fence},
};

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ, time_driver_impl};
use embassy_time_queue_utils::Queue;
use static_cell::StaticCell;

use crate::{
    Block, Result,
    gpt::GptInstance,
    timer_api::{CompareMatchChannel, TimerApi},
};
use ra_fsp_sys::generated::{
    FSP_INVALID_VECTOR, //
    R_BSP_IrqClearPending,
    R_BSP_IrqDisable,
    R_BSP_IrqEnable,
    R_BSP_IrqEnableNoClear,
    R_GPT0_Type,
    e_timer_event,
    gpt_extended_cfg_t,
    timer_callback_args_t,
};

// Clock timekeeping works with something we call "periods", which are time intervals
// of 2^31 ticks. The Clock counter value is 32 bits, so one "overflow cycle" is 2 periods.
//
// A `period` count is maintained in parallel to the Timer hardware `counter`, like this:
// - `period` and `counter` start at 0
// - `period` is incremented on overflow (at counter value 0)
// - `period` is incremented "midway" between overflows (at counter value 0x80000000)
//
// Therefore, when `period` is even, counter is in 0..0x7FFFFFFF. When odd, counter is in 0x8000000..0xFFFFFFFF
// This allows for now() to return the correct value even if it races an overflow.
//
// To get `now()`, `period` is read first, then `counter` is read. If the counter value matches
// the expected range for the `period` parity, we're done. If it doesn't, this means that
// a new period start has raced us between reading `period` and `counter`, so we assume the `counter` value
// corresponds to the next period.
//
// `period` is a 32bit integer, so It overflows on 2^32 * 2^31 / 30000000 seconds of uptime, which is 9749 years.
fn calc_now(period: u32, counter: u32) -> u64 {
    ((period as u64) << 31) + ((counter ^ ((period & 1) << 31)) as u64)
}

time_driver_impl!(
    static DRIVER_WRAPPER: GptTimerDriverWrapper = GptTimerDriverWrapper(AtomicPtr::new(null_mut()))
);

static DRIVER: StaticCell<GptTimerDriver> = StaticCell::new();

struct AlarmState {
    timestamp: Cell<u64>,
}

pub struct GptTimerDriver {
    /// number of 2^32 periods elapsed since boot
    period: AtomicU32,
    alarm: Mutex<AlarmState>,
    gpt: Mutex<RefCell<Pin<&'static mut GptInstance>>>,
    queue: Mutex<RefCell<Queue>>,
    // for fast access
    regs: *mut R_GPT0_Type,
    ext_cfg: *const gpt_extended_cfg_t,
}

struct GptTimerDriverWrapper(AtomicPtr<GptTimerDriver>);

unsafe impl Send for AlarmState {}

unsafe impl Send for GptTimerDriver {}
unsafe impl Sync for GptTimerDriver {}

impl Driver for GptTimerDriverWrapper {
    fn now(&self) -> u64 {
        let ptr = self.0.load(atomic::Ordering::Relaxed);
        match unsafe { ptr.as_ref() } {
            None => panic!("GPT Driver not initialized"),
            Some(drv) => drv.now(),
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        let ptr = self.0.load(atomic::Ordering::Relaxed);
        match unsafe { ptr.as_ref() } {
            None => panic!("GPT Driver not initialized"),
            Some(drv) => drv
                .schedule_wake(at, waker)
                .expect("Error scheduling wakeup"),
        }
    }
}

pub fn init(gpt: &'static mut GptInstance) -> Result<()> {
    critical_section::with(|cs| unsafe {
        let drv = Pin::static_mut(DRIVER.init(GptTimerDriver::new(gpt)));
        drv.as_ref().init(cs)?;
        let ptr = core::ptr::from_mut(drv.get_unchecked_mut());
        DRIVER_WRAPPER.0.store(ptr, atomic::Ordering::SeqCst);
        Ok(())
    })
}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
        }
    }
}

impl GptTimerDriver {
    pub fn new(gpt: &'static mut GptInstance) -> Self {
        let mut gpt = Pin::static_mut(gpt);
        let cfg = gpt.as_mut().instance().p_cfg;
        let ext_cfg: *const gpt_extended_cfg_t;

        unsafe {
            if (*cfg).cycle_end_irq == FSP_INVALID_VECTOR {
                panic!("cycle_end_irq invalid");
            }

            ext_cfg = (*cfg).p_extend.cast();
            if ext_cfg.is_null() {
                panic!(
                    "p_extend == null, so compare match interrupts are certainly not configured"
                );
            }

            let ext_cfg = (*cfg).p_extend.cast::<gpt_extended_cfg_t>();
            if (*ext_cfg).capture_a_irq == FSP_INVALID_VECTOR
                || (*ext_cfg).capture_b_irq == FSP_INVALID_VECTOR
            {
                panic!("capture_a_irq/capture_b_irq invalid");
            }
        }

        Self {
            regs: gpt.regs(),
            ext_cfg,
            period: AtomicU32::new(0),
            alarm: Mutex::new(AlarmState::new()),
            gpt: Mutex::new(RefCell::new(gpt)),
            queue: Mutex::new(RefCell::new(Queue::new())),
        }
    }

    unsafe fn init(self: Pin<&Self>, cs: CriticalSection) -> Result<()> {
        let mut gpt = self.gpt.borrow(cs).borrow_mut();

        if gpt
            .as_mut()
            .info_get()
            .expect("gpt.info_get() error")
            .clock_frequency as u64
            != TICK_HZ
        {
            panic!("GPT frequency not matching selected tick-hz-* feature");
        }

        unsafe {
            R_BSP_IrqDisable((*self.ext_cfg).capture_a_irq);
            R_BSP_IrqClearPending((*self.ext_cfg).capture_a_irq);
            R_BSP_IrqDisable((*self.ext_cfg).capture_b_irq);
            R_BSP_IrqClearPending((*self.ext_cfg).capture_b_irq);

            gpt.as_mut().stop()?;
            gpt.as_mut().reset()?;
            gpt.as_mut()
                .compare_match_set(0x80000000, CompareMatchChannel::A)?;

            let p_context = (&raw const *self).cast::<c_void>().cast_mut();

            gpt.as_mut()
                .callback_set(Some(Self::cb), p_context, core::ptr::null_mut())?;

            R_BSP_IrqEnable((*self.ext_cfg).capture_a_irq);

            gpt.as_mut().start().expect("Error starting GPT");
        }

        Ok(())
    }

    unsafe extern "C" fn cb(arg: *mut timer_callback_args_t) {
        critical_section::with(|cs| unsafe {
            let timer: &GptTimerDriver = &*(*arg).p_context.cast::<GptTimerDriver>();

            match (*arg).event {
                e_timer_event::TIMER_EVENT_CYCLE_END => Ok(timer.next_period()),
                // R_BSP_IrqClearPending((*timer.ext_cfg).capture_a_irq);
                e_timer_event::TIMER_EVENT_COMPARE_A => Ok(timer.next_period()),
                // R_BSP_IrqClearPending((*timer.ext_cfg).capture_b_irq);
                e_timer_event::TIMER_EVENT_COMPARE_B => timer.trigger_alarm(cs),
                _ => Ok(()),
            }
        })
        .expect("Error in callback handler");
    }

    fn next_period(&self) {
        // We only modify the period from the timer interrupt, so we know this can't race.
        let period = self.period.load(atomic::Ordering::Relaxed) + 1;
        self.period.store(period, atomic::Ordering::Relaxed);
        let t = (period as u64) << 31;

        critical_section::with(move |cs| {
            let alarm = self.alarm.borrow(cs);
            let at = alarm.timestamp.get();

            if at < t + 0xc0000000 {
                // just enable it. `set_alarm` has already set the correct CCR val.
                unsafe {
                    R_BSP_IrqEnableNoClear((*self.ext_cfg).capture_b_irq);
                }
            }
        })
    }

    fn trigger_alarm(&self, cs: CriticalSection) -> Result<()> {
        let mut next = self
            .queue
            .borrow(cs)
            .borrow_mut()
            .next_expiration(self.now());

        while !self.set_alarm(cs, next)? {
            next = self
                .queue
                .borrow(cs)
                .borrow_mut()
                .next_expiration(self.now());
        }

        Ok(())
    }

    fn now(&self) -> u64 {
        let period = self.period.load(atomic::Ordering::Relaxed);
        compiler_fence(atomic::Ordering::Acquire);
        let counter = unsafe { (&raw mut (*self.regs).__bindgen_anon_19.GTCNT).read_volatile() };
        calc_now(period, counter)
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) -> Result<()> {
        critical_section::with(|cs| -> Result<()> {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(cs, next)? {
                    next = queue.next_expiration(self.now());
                }
            }

            Ok(())
        })
    }

    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> Result<bool> {
        let mut gpt = self.gpt.borrow(cs).borrow_mut();
        let irq_capture_b = unsafe { (*self.ext_cfg).capture_b_irq };

        self.alarm.borrow(cs).timestamp.set(timestamp);

        let t = self.now();
        if timestamp <= t {
            // If alarm timestamp has passed the alarm will not fire.
            // Disarm the alarm and return `false` to indicate that.
            unsafe {
                R_BSP_IrqDisable(irq_capture_b);
            }

            self.alarm.borrow(cs).timestamp.set(u64::MAX);

            return Ok(false);
        }

        // Write the CCR value regardless of whether we're going to enable it now or not.
        // This way, when we enable it later, the right value is already set.

        gpt.as_mut()
            .compare_match_set(timestamp as u32, CompareMatchChannel::B)?;

        // Enable it if it'll happen soon. Otherwise, `next_period` will enable it.
        let diff = timestamp - t;
        unsafe {
            if diff < 0xc0000000 {
                R_BSP_IrqEnableNoClear(irq_capture_b);
            } else {
                R_BSP_IrqDisable(irq_capture_b);
            }
        }

        // Reevaluate if the alarm timestamp is still in the future
        let t = self.now();
        if timestamp <= t {
            // If alarm timestamp has passed since we set it, we have a race condition and
            // the alarm may or may not have fired.
            // Disarm the alarm and return `false` to indicate that.
            // It is the caller's responsibility to handle this ambiguity.
            unsafe {
                R_BSP_IrqDisable(irq_capture_b);
            }

            self.alarm.borrow(cs).timestamp.set(u64::MAX);

            return Ok(false);
        }

        // We're confident the alarm will ring in the future.
        Ok(true)
    }
}
