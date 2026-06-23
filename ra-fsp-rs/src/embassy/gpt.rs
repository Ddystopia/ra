use core::{cell::RefCell, pin::Pin};

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use ra_fsp_sys::generated::{e_fsp_err, e_timer_event, timer_event_t};

use crate::{
    Callback, DriverBox, Result,
    gpt::{Channel, Gpt, IsrPrototype},
    gpt_clock::{self, GptTimerStorage, Storage, TimerStateExt},
    state_markers::Opened,
    timer_api::TimerApi,
};

pub use embassy_time_driver::time_driver_impl;

/// Shared GPT-backed `embassy-time` timekeeping: a 64-bit clock extended from the
/// 32-bit counter, plus the wake/alarm queue. Both public driver types wrap this;
/// they differ only in how the FSP callback is registered and how the GPT ISRs
/// are dispatched.
struct GptTimekeeper<C: 'static> {
    timer: GptTimerStorage<C>,
    queue: Mutex<RefCell<Queue>>,
}

impl<C: Channel> GptTimekeeper<C> {
    const fn new() -> Self {
        Self {
            timer: GptTimerStorage::new(),
            queue: Mutex::new(RefCell::new(Queue::new())),
        }
    }

    fn now(&self) -> u64 {
        self.timer.timer_state.must_get().now()
    }

    fn trigger_alarm<T: TimerApi>(&self, cs: CriticalSection, mut gpt: Pin<&mut T>) -> Result<()> {
        let timer = self.timer.timer_state.must_get();
        let mut queue = self.queue.borrow_ref_mut(cs);
        let mut next = queue.next_expiration(timer.now());

        while !timer.set_alarm(cs, gpt.as_mut(), next)? {
            next = queue.next_expiration(timer.now());
        }

        Ok(())
    }

    fn schedule_wake<T: TimerApi>(
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

    /// Body of [`Driver::schedule_wake`], shared by both wrappers; reaches the
    /// `Gpt` through [`GptTimerStorage::gpt`].
    fn driver_schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        // FIXME(latency): opens a global critical section that spans
        // `TimerState::set_alarm`; see the FIXME there for details.
        critical_section::with(|cs| {
            let mut borrow = self.timer.gpt.borrow_ref_mut(cs);
            let gpt = borrow.as_mut().expect("Driver not initialized").as_mut();

            self.schedule_wake(cs, gpt, at, waker)
                .expect("Error scheduling wakeup")
        })
    }
}

/// Verify the GPT counts at the embassy `TICK_HZ`.
fn check_clock<C: Channel + 'static>(gpt: Pin<&mut Gpt<'static, C, Opened>>) -> Result<()> {
    if gpt.info_get()?.clock_frequency as u64 != TICK_HZ {
        log::error!("GPT frequency not matching selected tick-hz-* feature");
        return Err(e_fsp_err::FSP_ERR_ASSERTION);
    }
    Ok(())
}

/// `embassy-time` driver for a GPT channel, dispatched with
/// [`handle_isr`](Self::handle_isr).
///
/// `handle_isr` gates on the active IRQ *number*, so it dispatches correctly no
/// matter how the vector table is wired, including behind a trampoline such as
/// an RTIC `#[task(binds = ..)]`. Use [`EmbassyGptDirectTimerDriver`] on a
/// direct-vector install that wants the vector-verified
/// [`IsrPrototype::call_fsp_isr_handler`].
///
/// [`IsrPrototype::call_fsp_isr_handler`]: crate::gpt::IsrPrototype::call_fsp_isr_handler
pub struct EmbassyGptTimerDriver<C: 'static>(GptTimekeeper<C>);

impl<C: Channel> EmbassyGptTimerDriver<C> {
    pub const fn new() -> Self {
        Self(GptTimekeeper::new())
    }
}

impl<C: Channel + 'static> EmbassyGptTimerDriver<C> {
    /// Open the GPT clock. Drive the three GPT IRQs (cycle-end, capture A,
    /// capture B) with [`handle_isr`](Self::handle_isr) from their handlers.
    pub fn start(&'static self, mut gpt: DriverBox<Gpt<'static, C, Opened>>) -> Result<()> {
        check_clock(gpt.as_mut())?;
        gpt_clock::start_with_block::<C, Self>(gpt, self)
    }

    /// Dispatch a GPT ISR from its bound interrupt handler (e.g. an RTIC
    /// hardware task). [`Gpt::handle_isr`] no-ops unless it runs in the channel's
    /// configured IRQ, so this needs no `unsafe` and works under any vector
    /// wiring.
    ///
    /// The block-taking callback reaches the `Gpt` through the block passed in,
    /// so this single `borrow_ref_mut` is the only borrow on the callback path.
    ///
    /// [`Gpt::handle_isr`]: crate::gpt::Gpt::handle_isr
    pub fn handle_isr(&'static self, isr_prototype: IsrPrototype) {
        critical_section::with(|cs| {
            let mut borrow = self.0.timer.gpt.borrow_ref_mut(cs);
            let Some(gpt) = borrow.as_mut() else { return };

            if matches!(isr_prototype, IsrPrototype::CompareB) {
                // Lean alarm path: clear the ICU IR flag and drive the timer
                // queue directly, skipping the FSP capture-B ISR dispatch -- its
                // context lookup, `callback_args` build, trampoline and event
                // match. For a periodic compare match the FSP ISR does no more
                // than this (it never touches GTST); this mirrors the body of
                // `call_with_block`'s `TIMER_EVENT_COMPARE_B` arm.
                if let Some(irq) = crate::utils::current_irq_get() {
                    crate::icu::irq_status_clear(irq);
                }
                self.0
                    .trigger_alarm(cs, gpt.as_mut())
                    .expect("Error in alarm handler");
            } else {
                // Period events (overflow / compare-A) keep the FSP ISR path.
                gpt.as_mut().handle_isr(isr_prototype);
            }
        });
    }
}

impl<C: Channel + 'static> Driver for EmbassyGptTimerDriver<C> {
    fn now(&self) -> u64 {
        self.0.now()
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        self.0.driver_schedule_wake(at, waker)
    }
}

impl<C> Storage<C> for EmbassyGptTimerDriver<C> {
    fn storage(&'static self) -> &'static GptTimerStorage<C> {
        &self.0.timer
    }
}

impl<C: Channel + 'static> Callback<timer_event_t, Gpt<'static, C, Opened>>
    for EmbassyGptTimerDriver<C>
{
    #[inline(always)]
    fn call_with_block(
        this: &Self,
        block: Pin<&mut Gpt<'static, C, Opened>>,
        event: e_timer_event,
    ) {
        let state = this.0.timer.timer_state.must_get();
        match event {
            e_timer_event::TIMER_EVENT_CYCLE_END => Ok(state.next_period()),
            e_timer_event::TIMER_EVENT_COMPARE_A => Ok(state.next_period()),
            e_timer_event::TIMER_EVENT_COMPARE_B => {
                critical_section::with(|cs| this.0.trigger_alarm(cs, block))
            }
            _ => Ok(()),
        }
        .expect("Error in callback handler");
    }
}

/// `embassy-time` driver for a GPT channel, dispatched with
/// [`IsrPrototype::call_fsp_isr_handler`].
///
/// That dispatcher verifies the active interrupt vector points at the FSP ISR,
/// so it is sound to call from any context, and matches on a direct-vector
/// install where the FSP ISR sits in the vector table. Behind an indirected
/// vector (e.g. an RTIC `#[task(binds = ..)]` dispatcher) the slot holds the
/// trampoline, not the FSP ISR, so dispatch no-ops; use
/// [`EmbassyGptTimerDriver`] there.
///
/// [`IsrPrototype::call_fsp_isr_handler`]: crate::gpt::IsrPrototype::call_fsp_isr_handler
pub struct EmbassyGptDirectTimerDriver<C: 'static>(GptTimekeeper<C>);

impl<C: Channel> EmbassyGptDirectTimerDriver<C> {
    pub const fn new() -> Self {
        Self(GptTimekeeper::new())
    }
}

impl<C: Channel + 'static> EmbassyGptDirectTimerDriver<C> {
    /// Open the GPT clock. Drive the three GPT IRQs (cycle-end, capture A,
    /// capture B) with [`IsrPrototype::call_fsp_isr_handler`] from their handlers.
    ///
    /// [`IsrPrototype::call_fsp_isr_handler`]: crate::gpt::IsrPrototype::call_fsp_isr_handler
    pub fn start(&'static self, mut gpt: DriverBox<Gpt<'static, C, Opened>>) -> Result<()> {
        check_clock(gpt.as_mut())?;
        gpt_clock::start::<C, Self>(gpt, self)
    }
}

impl<C: Channel + 'static> Driver for EmbassyGptDirectTimerDriver<C> {
    fn now(&self) -> u64 {
        self.0.now()
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        self.0.driver_schedule_wake(at, waker)
    }
}

impl<C> Storage<C> for EmbassyGptDirectTimerDriver<C> {
    fn storage(&'static self) -> &'static GptTimerStorage<C> {
        &self.0.timer
    }
}

impl<C: Channel + 'static> Callback<timer_event_t> for EmbassyGptDirectTimerDriver<C> {
    #[inline(always)]
    fn call(this: &Self, event: e_timer_event) {
        let state = this.0.timer.timer_state.must_get();
        let gpt = &this.0.timer.gpt;
        match event {
            e_timer_event::TIMER_EVENT_CYCLE_END => Ok(state.next_period()),
            e_timer_event::TIMER_EVENT_COMPARE_A => Ok(state.next_period()),
            e_timer_event::TIMER_EVENT_COMPARE_B => critical_section::with(|cs| {
                this.0
                    .trigger_alarm(cs, gpt.borrow_ref_mut(cs).as_mut().unwrap().as_mut())
            }),
            _ => Ok(()),
        }
        .expect("Error in callback handler");
    }
}
