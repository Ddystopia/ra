use core::pin::Pin;

use ra_fsp_sys::generated::{e_timer_event, timer_event_t};
use rtic_monotonics::{TimerQueueBackend, rtic_time::timer_queue::TimerQueue};

use crate::{
    Callback, Result,
    gpt::Gpt,
    gpt_clock::{self, GptTimerDriver, Storage, TimerState},
    state_markers::Opened,
};

static TIMER_QUEUE: TimerQueue<TimerBackend> = TimerQueue::new();

pub struct TimerBackend;

#[derive(Default)]
struct RticGptExtension;

pub fn start(gpt: Pin<&'static mut Gpt<Opened>>) -> Result<()> {
    gpt_clock::start::<RticGptExtension>(gpt)
}

impl Storage for RticGptExtension {
    fn driver() -> &'static GptTimerDriver<RticGptExtension> {
        static DRIVER: GptTimerDriver<RticGptExtension> = GptTimerDriver::<RticGptExtension>::new();

        &DRIVER
    }
}

impl TimerBackend {
    fn timer() -> &'static GptTimerDriver<RticGptExtension> {
        RticGptExtension::driver()
    }
}

impl TimerQueueBackend for TimerBackend {
    type Ticks = u64;

    fn now() -> Self::Ticks {
        // todo: check asm or bench, maybe critical section + RefCell is better
        match TimerBackend::timer().timer_state.get() {
            None => panic!("GPT Driver not initialized"),
            Some(timer_state) => timer_state.now(),
        }
    }

    fn set_compare(instant: Self::Ticks) {
        critical_section::with(|cs| {
            if let Some(timer) = RticGptExtension::driver().timer_state.get() {
                let mut borrow = RticGptExtension::driver().gpt.borrow_ref_mut(cs);
                let gpt = borrow.as_mut().unwrap().as_mut();
                timer
                    .set_alarm(cs, gpt, instant)
                    .expect("Error setting alarm");
            }
        })
    }

    fn clear_compare_flag() {
        critical_section::with(|cs| {
            if let Some(timer) = RticGptExtension::driver().timer_state.get() {
                timer.reset_alarm(cs);
            }
        })
    }

    fn pend_interrupt() {
        // SAFETY: docs say it is always safe to call
        unsafe { TIMER_QUEUE.on_monotonic_interrupt() }
    }

    fn timer_queue() -> &'static TimerQueue<Self> {
        &TIMER_QUEUE
    }
}

impl Callback<timer_event_t> for TimerState<RticGptExtension> {
    #[inline(always)]
    fn call(timer: &Self, event: e_timer_event) {
        match event {
            e_timer_event::TIMER_EVENT_CYCLE_END => timer.next_period(),
            // R_BSP_IrqClearPending((*timer.ext_cfg).capture_a_irq);
            e_timer_event::TIMER_EVENT_COMPARE_A => timer.next_period(),
            // R_BSP_IrqClearPending((*timer.ext_cfg).capture_b_irq);
            e_timer_event::TIMER_EVENT_COMPARE_B => TimerBackend::pend_interrupt(),
            _ => (),
        }
    }
}
