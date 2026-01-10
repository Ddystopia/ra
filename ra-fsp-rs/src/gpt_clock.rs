use core::{
    cell::{Cell, RefCell},
    pin::Pin,
    sync::atomic::{self, AtomicU32, Ordering, compiler_fence},
};

use atomic_once_cell::AtomicOnceCell;
use cortex_m::peripheral::NVIC;
use critical_section::{CriticalSection, Mutex};

use crate::{
    Callback, DriverBox, Result, gpt::{Channel, Gpt}, pac, state_markers::Opened, timer_api::TimerApi
};

#[allow(unused_imports)]
use ra_fsp_sys::generated::R_GPT0_Type;
use ra_fsp_sys::generated::{e_fsp_err, e_timer_compare_match, timer_event_t};

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

pub struct AlarmState {
    pub timestamp: Cell<u64>,
}

pub struct TimerState<C> {
    /// number of 2^32 periods elapsed since boot
    period: AtomicU32,
    alarm: Mutex<AlarmState>,
    capture_a_irq: pac::Interrupt,
    capture_b_irq: pac::Interrupt,
    _marker: core::marker::PhantomData<fn(C) -> C>,
}

pub struct GptTimerStorage<C: 'static> {
    pub gpt: Mutex<RefCell<Option<DriverBox<Gpt<'static, C, Opened>>>>>,
    pub timer_state: AtomicOnceCell<TimerState<C>>,
}

pub trait Storage<C: 'static> {
    fn storage(&'static self) -> &'static GptTimerStorage<C>;
}

pub fn start<C: Channel + 'static, Cb: Callback<timer_event_t> + Storage<C>>(
    gpt: DriverBox<Gpt<'static, C, Opened>>,
    callback: &'static Cb,
) -> Result<()> {
    if gpt.capture_a_irq().is_none() {
        log::error!("cycle_end_irq invalid");

        return Err(e_fsp_err::FSP_ERR_INVALID_ARGUMENT);
    }

    let Some((capture_a_irq, capture_b_irq)) = gpt.capture_a_irq().zip(gpt.capture_b_irq()) else {
        log::error!("capture_a_irq/capture_b_irq invalid");

        return Err(e_fsp_err::FSP_ERR_INVALID_ARGUMENT);
    };

    let storage = callback.storage();

    if let Err(_) = storage.timer_state.set(TimerState {
        capture_a_irq,
        capture_b_irq,
        period: AtomicU32::new(0),
        alarm: Mutex::new(AlarmState::new()),
        _marker: core::marker::PhantomData,
    }) {
        log::error!("Embassy GPT Timer already started");
        return Err(e_fsp_err::FSP_ERR_ALREADY_OPEN);
    }

    let timer_state = &storage.timer_state.get().unwrap();

    let channel = e_timer_compare_match::TIMER_COMPARE_MATCH_A;
    let capture_a_irq = timer_state.capture_a_irq;
    let capture_b_irq = timer_state.capture_b_irq;

    critical_section::with(|cs| {
        *storage.gpt.borrow_ref_mut(cs) = Some(gpt);

        let mut borrow = storage.gpt.borrow_ref_mut(cs);
        let mut gpt = borrow.as_mut().unwrap().as_mut();

        timer_state.mask_a();
        NVIC::unpend(capture_a_irq);
        timer_state.mask_b();
        NVIC::unpend(capture_b_irq);

        gpt.as_mut().stop()?;
        gpt.as_mut().stop()?;
        gpt.as_mut().reset()?;
        gpt.as_mut().compare_match_set(0x80000000, channel)?;
        gpt.as_mut().callback_set_static(callback)?;

        NVIC::unpend(capture_a_irq);
        timer_state.unmask_a();

        gpt.as_mut().start()?;

        Ok(())
    })
}

pub trait TimerStateExt<C> {
    fn must_get(&self) -> &TimerState<C>;
}

impl<C> TimerStateExt<C> for AtomicOnceCell<TimerState<C>> {
    fn must_get(&self) -> &TimerState<C> {
        match self.get() {
            Some(state) => state,
            None => panic!("GPT TimerState not initialized"),
        }
    }
}

impl<C: Channel> TimerState<C> {
    /// Dianables capture a interrupt
    pub fn mask_a(&self) {
        NVIC::mask(self.capture_a_irq)
    }
    /// Dianables capture b interrupt
    pub fn mask_b(&self) {
        NVIC::mask(self.capture_b_irq)
    }
    /// Enables capture a interrupt
    pub fn unmask_a(&self) {
        unsafe { NVIC::unmask(self.capture_a_irq) }
    }
    /// Enables capture b interrupt
    pub fn unmask_b(&self) {
        // todo: make sure that nothing else has that irq so
        //       we will not tamper with it
        unsafe { NVIC::unmask(self.capture_b_irq) }
    }

    #[inline(always)]
    pub fn now(&self) -> u64 {
        let period = self.period.load(atomic::Ordering::Relaxed);
        compiler_fence(atomic::Ordering::Acquire);
        let counter = C::gtcnt();
        calc_now(period, counter)
    }

    pub fn next_period(&self) {
        // We only modify the period from the timer interrupt, so we know this can't race.
        let period = self.period.load(Ordering::Relaxed) + 1;
        self.period.store(period, Ordering::Relaxed);
        let t = (period as u64) << 31;

        critical_section::with(move |cs| {
            let alarm = self.alarm.borrow(cs);
            let at = alarm.timestamp.get();

            if at < t + 0xc0000000 {
                // just enable it. `set_alarm` has already set the correct CCR val.
                self.unmask_b();
            }
        })
    }

    #[allow(dead_code)] // used in rtic impl
    pub fn reset_alarm(&self, cs: CriticalSection) {
        self.alarm.borrow(cs).timestamp.set(u64::MAX);
        NVIC::mask(self.capture_b_irq);
    }

    pub fn set_alarm<T: TimerApi>(
        &self,
        cs: CriticalSection,
        mut gpt: Pin<&mut T>,
        timestamp: u64,
    ) -> Result<bool> {
        self.alarm.borrow(cs).timestamp.set(timestamp);

        let t = self.now();
        if timestamp <= t {
            // If alarm timestamp has passed the alarm will not fire.
            // Disarm the alarm and return `false` to indicate that.
            self.mask_b();

            self.alarm.borrow(cs).timestamp.set(u64::MAX);

            return Ok(false);
        }

        // Write the CCR value regardless of whether we're going to enable it now or not.
        // This way, when we enable it later, the right value is already set.

        gpt.as_mut().compare_match_set(
            timestamp as u32,
            e_timer_compare_match::TIMER_COMPARE_MATCH_B,
        )?;

        // Enable it if it'll happen soon. Otherwise, `next_period` will enable it.
        let diff = timestamp - t;

        if diff < 0xc0000000 {
            self.unmask_b();
        } else {
            self.mask_b();
        }

        // Reevaluate if the alarm timestamp is still in the future
        let t = self.now();
        if timestamp <= t {
            // If alarm timestamp has passed since we set it, we have a race condition and
            // the alarm may or may not have fired.
            // Disarm the alarm and return `false` to indicate that.
            // It is the caller's responsibility to handle this ambiguity.
            self.mask_b();

            self.alarm.borrow(cs).timestamp.set(u64::MAX);

            return Ok(false);
        }

        // We're confident the alarm will ring in the future.
        Ok(true)
    }
}

impl<C: 'static> GptTimerStorage<C> {
    pub const fn new() -> Self {
        Self {
            gpt: Mutex::new(RefCell::new(None)),
            timer_state: AtomicOnceCell::new(),
        }
    }
}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
        }
    }
}
