use core::{cell::RefCell, pin::Pin};

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use ra_fsp_sys::generated::{e_fsp_err, e_timer_event, timer_event_t};

use crate::{
    Callback, Result,
    gpt::{Channel, Gpt},
    gpt_clock::{self, GptTimerStorage, Storage, TimerStateExt},
    state_markers::Opened,
    timer_api::TimerApi,
};

pub struct EmbassyGptTimerDriver<C: 'static> {
    timer: GptTimerStorage<C>,
    queue: Mutex<RefCell<Queue>>,
}

impl<C: Channel + 'static> EmbassyGptTimerDriver<C> {
    pub fn start(&'static self, mut gpt: Pin<&'static mut Gpt<'static, C, Opened>>) -> Result<()> {
        if gpt.as_mut().info_get()?.clock_frequency as u64 != TICK_HZ {
            log::error!("GPT frequency not matching selected tick-hz-* feature");
            return Err(e_fsp_err::FSP_ERR_ASSERTION);
        }

        gpt_clock::start::<C, EmbassyGptTimerDriver<C>>(gpt, self)
    }
}

impl<C: 'static> Default for EmbassyGptTimerDriver<C> {
    fn default() -> Self {
        EmbassyGptTimerDriver {
            timer: GptTimerStorage::new(),
            queue: Mutex::new(RefCell::new(Queue::new())),
        }
    }
}

impl<C: Channel> EmbassyGptTimerDriver<C> {
    pub fn trigger_alarm<T: TimerApi>(
        &self,
        cs: CriticalSection,
        mut gpt: Pin<&mut T>,
    ) -> Result<()> {
        let timer = self.timer.timer_state.must_get();
        let mut queue = self.queue.borrow_ref_mut(cs);
        let mut next = queue.next_expiration(timer.now());

        while !timer.set_alarm(cs, gpt.as_mut(), next)? {
            next = queue.next_expiration(timer.now());
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
        let timer = self.timer.timer_state.must_get();
        let mut queue = self.queue.borrow_ref_mut(cs);

        if queue.schedule_wake(at, waker) {
            let mut next = queue.next_expiration(timer.now());
            while !timer.set_alarm(cs, gpt.as_mut(), next)? {
                next = queue.next_expiration(timer.now());
            }
        }

        Ok(())
    }
}

impl<C: Channel + 'static> Driver for EmbassyGptTimerDriver<C> {
    fn now(&self) -> u64 {
        self.timer.timer_state.must_get().now()
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut borrow = self.timer.gpt.borrow_ref_mut(cs);
            let gpt = borrow.as_mut().expect("Driver not initialized").as_mut();

            self.schedule_wake(cs, gpt, at, waker)
                .expect("Error scheduling wakeup")
        })
    }
}

impl<C> Storage<C> for EmbassyGptTimerDriver<C> {
    fn storage(&'static self) -> &'static GptTimerStorage<C> {
        &self.timer
    }
}

impl<C: Channel + 'static> Callback<timer_event_t> for EmbassyGptTimerDriver<C> {
    #[inline(always)]
    fn call(this: &Self, event: e_timer_event) {
        let state = this.timer.timer_state.must_get();
        let gpt = &this.timer.gpt;
        critical_section::with(|cs| match event {
            e_timer_event::TIMER_EVENT_CYCLE_END => Ok(state.next_period()),
            e_timer_event::TIMER_EVENT_COMPARE_A => Ok(state.next_period()),
            e_timer_event::TIMER_EVENT_COMPARE_B => {
                this.trigger_alarm(cs, gpt.borrow_ref_mut(cs).as_mut().unwrap().as_mut())
            }
            _ => Ok(()),
        })
        .expect("Error in callback handler");
    }
}
