use ra_fsp_sys::generated::e_timer_event;

use crate::{
    gpt::Channel,
    gpt_clock::{GptTimerStorage, TimerStateExt},
};

pub struct RticGptTimerDriver<C: 'static>(pub GptTimerStorage<C>);

impl<C: Channel> RticGptTimerDriver<C> {
    pub const fn new() -> Self {
        Self(GptTimerStorage::new())
    }
}

#[macro_export]
macro_rules! gpt_monotonic {
    (static $static_name:ident: $name:ident<$channel:path>) => {
        pub struct $name {
            queue: ::rtic_monotonics::rtic_time::timer_queue::TimerQueue<Self>,
            storage: $crate::rtic::gpt::RticGptTimerDriver<$channel>,
        }
        static $static_name: $name = $name {
            queue: ::rtic_monotonics::rtic_time::timer_queue::TimerQueue::new(),
            storage: $crate::rtic::gpt::RticGptTimerDriver::new(),
        };
        impl $name {
            pub fn start(
                &'static self,
                gpt: $crate::DriverBox<
                    $crate::gpt::Gpt<'static, $channel, $crate::state_markers::Opened>,
                >,
            ) -> $crate::Result<()> {
                $crate::gpt_clock::start::<$channel, Self>(gpt, self)
            }
            fn pend() {
                // SAFETY: always safe per rtic-time docs; reached from the
                // capture-B ISR (COMPARE_B event, possibly software-pended by
                // `pend_interrupt`), the monotonic's interrupt.
                unsafe { $static_name.queue.on_monotonic_interrupt() }
            }
        }
        impl $crate::Callback<$crate::sys::generated::timer_event_t> for $name {
            #[inline(always)]
            fn call(this: &Self, event: $crate::sys::generated::e_timer_event) {
                $crate::rtic::gpt::gpt_callback(event, &this.storage.0, $name::pend)
            }
        }
        impl $crate::gpt_clock::Storage<$channel> for $name {
            fn storage(&'static self) -> &'static $crate::gpt_clock::GptTimerStorage<$channel> {
                &self.storage.0
            }
        }

        impl ::rtic_monotonics::TimerQueueBackend for $name {
            type Ticks = u64;

            fn pend_interrupt() {
                $static_name.storage.pend_alarm()
            }

            fn timer_queue() -> &'static ::rtic_monotonics::rtic_time::timer_queue::TimerQueue<Self>
            {
                &$static_name.queue
            }

            fn now() -> Self::Ticks {
                $static_name.storage.now()
            }

            fn set_compare(instant: Self::Ticks) {
                $static_name.storage.set_compare(instant)
            }

            fn clear_compare_flag() {
                $static_name.storage.clear_compare_flag()
            }
        }
    };
}

impl<C: Channel + 'static> RticGptTimerDriver<C> {
    pub fn now(&self) -> u64 {
        // todo: check asm or bench, maybe critical section + RefCell is better
        match self.0.timer_state.get() {
            None => panic!("GPT Driver not initialized"),
            Some(timer_state) => timer_state.now(),
        }
    }

    pub fn set_compare(&self, instant: u64) {
        critical_section::with(|cs| {
            if let Some(timer) = self.0.timer_state.get() {
                let mut borrow = self.0.gpt.borrow_ref_mut(cs);
                let gpt = borrow.as_mut().unwrap().as_mut();
                timer
                    .set_alarm(cs, gpt, instant)
                    .expect("Error setting alarm");
            }
        })
    }

    pub fn clear_compare_flag(&self) {
        critical_section::with(|cs| {
            if let Some(timer) = self.0.timer_state.get() {
                timer.reset_alarm(cs);
            }
        })
    }

    /// Pends the capture-B IRQ so `on_monotonic_interrupt` runs in the
    /// monotonic's ISR (see [`TimerState::pend_alarm`]).
    ///
    /// [`TimerState::pend_alarm`]: crate::gpt_clock::TimerState::pend_alarm
    pub fn pend_alarm(&self) {
        self.0.timer_state.must_get().pend_alarm()
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn gpt_callback<C: Channel + 'static>(
    event: e_timer_event,
    storage: &GptTimerStorage<C>,
    pend_interrupt: impl Fn(),
) {
    let state = storage.timer_state.must_get();
    match event {
        e_timer_event::TIMER_EVENT_CYCLE_END => state.next_period(),
        e_timer_event::TIMER_EVENT_COMPARE_A => state.next_period(),
        e_timer_event::TIMER_EVENT_COMPARE_B => (pend_interrupt)(),
        _ => (),
    }
}
