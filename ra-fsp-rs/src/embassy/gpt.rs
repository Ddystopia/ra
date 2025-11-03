use core::{cell::RefCell, pin::Pin};

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ, time_driver_impl};
use embassy_time_queue_utils::Queue;
use ra_fsp_sys::generated::{e_fsp_err, e_timer_event, timer_event_t};

use crate::{
    Callback, Result,
    gpt::Gpt,
    gpt_clock::{self, GptTimerDriver, Storage, TimerState},
    state_markers::Opened,
    timer_api::TimerApi,
};

time_driver_impl!(
    static DRIVER: GptTimerDriver<EmbassyGptStorage> = GptTimerDriver::new()
);

struct EmbassyGptStorage(Mutex<RefCell<Queue>>);

pub fn start(mut gpt: Pin<&'static mut Gpt<Opened>>) -> Result<()> {
    if gpt.as_mut().info_get()?.clock_frequency as u64 != TICK_HZ {
        log::error!("GPT frequency not matching selected tick-hz-* feature");
        return Err(e_fsp_err::FSP_ERR_ASSERTION);
    }

    gpt_clock::start::<EmbassyGptStorage>(gpt)
}

impl Default for EmbassyGptStorage {
    fn default() -> Self {
        EmbassyGptStorage(Mutex::new(RefCell::new(Queue::new())))
    }
}

impl EmbassyGptStorage {
    fn borrow_mut<'a>(&'a self, cs: CriticalSection<'a>) -> core::cell::RefMut<'a, Queue> {
        self.0.borrow_ref_mut(cs)
    }
}

impl Storage for EmbassyGptStorage {
    fn driver() -> &'static GptTimerDriver<EmbassyGptStorage> {
        &DRIVER
    }
}

impl TimerState<EmbassyGptStorage> {
    pub fn trigger_alarm<T: TimerApi>(
        &self,
        cs: CriticalSection,
        mut gpt: Pin<&mut T>,
    ) -> Result<()> {
        let mut next = self.ext.borrow_mut(cs).next_expiration(self.now());

        while !self.set_alarm(cs, gpt.as_mut(), next)? {
            next = self.ext.borrow_mut(cs).next_expiration(self.now());
        }

        Ok(())
    }

    pub fn schedule_wake<T: TimerApi>(
        &self,
        cs: CriticalSection<'_>,
        mut gpt: Pin<&mut T>,
        at: u64,
        waker: &core::task::Waker,
    ) -> Result<()> {
        let mut queue = self.ext.borrow_mut(cs);

        if queue.schedule_wake(at, waker) {
            let mut next = queue.next_expiration(self.now());
            while !self.set_alarm(cs, gpt.as_mut(), next)? {
                next = queue.next_expiration(self.now());
            }
        }

        Ok(())
    }
}

impl Driver for GptTimerDriver<EmbassyGptStorage> {
    #[unsafe(export_name = "ddystopia_gpt_now_disasm")]
    fn now(&self) -> u64 {
        // todo: check asm or bench, maybe critical section + RefCell is better
        match self.timer_state.get() {
            None => panic!("GPT Driver not initialized"),
            Some(timer_state) => timer_state.now(),
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| match self.timer_state.get() {
            None => panic!("GPT Driver not initialized"),
            Some(timer_state) => {
                let mut borrow = self.gpt.borrow_ref_mut(cs);
                let gpt = borrow.as_mut().expect("Driver not initialized").as_mut();

                timer_state
                    .schedule_wake(cs, gpt, at, waker)
                    .expect("Error scheduling wakeup")
            }
        })
    }
}

impl Callback<timer_event_t> for TimerState<EmbassyGptStorage> {
    #[inline(always)]
    fn call(timer: &Self, event: e_timer_event) {
        critical_section::with(|cs| match event {
            e_timer_event::TIMER_EVENT_CYCLE_END => Ok(timer.next_period()),
            // R_BSP_IrqClearPending((*timer.ext_cfg).capture_a_irq);
            e_timer_event::TIMER_EVENT_COMPARE_A => Ok(timer.next_period()),
            // R_BSP_IrqClearPending((*timer.ext_cfg).capture_b_irq);
            e_timer_event::TIMER_EVENT_COMPARE_B => {
                timer.trigger_alarm(cs, DRIVER.gpt.borrow_ref_mut(cs).as_mut().unwrap().as_mut())
            }
            _ => Ok(()),
        })
        .expect("Error in callback handler");
    }
}
