use ra_fsp_sys::generated::e_timer_event;

use crate::{
    gpt::{Channel, IsrPrototype},
    gpt_clock::{GptTimerStorage, TimerStateExt},
    utils,
};

pub struct RticGptTimerDriver<C: 'static>(pub GptTimerStorage<C>);

impl<C: Channel> RticGptTimerDriver<C> {
    pub const fn new() -> Self {
        Self(GptTimerStorage::new())
    }
}

/// Define a `rtic-monotonics` monotonic backed by a GPT timer channel.
///
/// Expands to a public unit type `$name` implementing
/// [`rtic_monotonics::TimerQueueBasedMonotonic`] (so `now`, `delay` and
/// `delay_until` come from the blanket [`Monotonic`] impl) at the given tick
/// rate, which must equal the GPT count rate. The timer-queue backend and its
/// static live in a private `const` block, so `$name` is the only name this
/// introduces and the macro may be invoked more than once per module.
///
/// Open the channel and hand it to [`$name::start`]; drive the three GPT IRQs
/// (cycle-end, capture A, capture B) with [`$name::handle_isr`] from their bound
/// interrupt handlers (e.g. RTIC `#[task(binds = ..)]`).
///
/// ```ignore
/// ra_fsp_rs::gpt_monotonic!(Mono<GPT329>, 30_000_000);
/// ```
///
/// [`Monotonic`]: rtic_monotonics::Monotonic
#[macro_export]
macro_rules! gpt_monotonic {
    ($name:ident<$channel:path>, $tick_rate_hz:expr) => {
        /// A `rtic-monotonics` monotonic backed by a GPT timer channel. `now`,
        /// `delay` and `delay_until` come from the blanket
        /// [`Monotonic`](::rtic_monotonics::Monotonic) impl; drive its GPT IRQs
        /// with [`handle_isr`](Self::handle_isr).
        pub struct $name;

        const _: () = {
            // ZST marker carrying the `TimerQueueBackend` impl; the timer queue
            // and the GPT storage live in their own statics (mirrors
            // rtic-monotonics' `SystickBackend`, which `TimerQueue::initialize`
            // takes by value). `pub` so it does not leak as a private type
            // through `$name`'s public `TimerQueueBasedMonotonic::Backend`; the
            // anonymous `const` keeps it unnameable from outside.
            pub struct Backend;

            static TIMER_QUEUE: ::rtic_monotonics::rtic_time::timer_queue::TimerQueue<Backend> =
                ::rtic_monotonics::rtic_time::timer_queue::TimerQueue::new();
            static STORAGE: $crate::rtic::gpt::RticGptTimerDriver<$channel> =
                $crate::rtic::gpt::RticGptTimerDriver::new();
            static BACKEND: Backend = Backend;

            impl Backend {
                fn pend() {
                    // SAFETY: always safe per rtic-time docs; reached from the
                    // capture-B ISR (COMPARE_B event, possibly software-pended by
                    // `pend_interrupt`), the monotonic's interrupt.
                    unsafe { TIMER_QUEUE.on_monotonic_interrupt() }
                }
            }

            impl $crate::Callback<$crate::sys::generated::timer_event_t> for Backend {
                #[inline(always)]
                fn call(_this: &Self, event: $crate::sys::generated::e_timer_event) {
                    $crate::rtic::gpt::gpt_callback(event, &STORAGE.0, Backend::pend)
                }
            }

            impl $crate::gpt_clock::Storage<$channel> for Backend {
                fn storage(&'static self) -> &'static $crate::gpt_clock::GptTimerStorage<$channel> {
                    &STORAGE.0
                }
            }

            impl ::rtic_monotonics::TimerQueueBackend for Backend {
                type Ticks = u64;

                fn pend_interrupt() {
                    STORAGE.pend_alarm()
                }

                fn timer_queue()
                -> &'static ::rtic_monotonics::rtic_time::timer_queue::TimerQueue<Backend> {
                    &TIMER_QUEUE
                }

                fn now() -> Self::Ticks {
                    STORAGE.now()
                }

                fn set_compare(instant: Self::Ticks) {
                    STORAGE.set_compare(instant)
                }

                fn clear_compare_flag() {
                    STORAGE.clear_compare_flag()
                }
            }

            impl $name {
                /// Start the monotonic over an opened GPT channel. Drive the
                /// three GPT IRQs (cycle-end, capture A, capture B) with
                /// [`handle_isr`](Self::handle_isr) from their bound interrupt
                /// handlers.
                pub fn start(
                    gpt: $crate::DriverBox<
                        $crate::gpt::Gpt<'static, $channel, $crate::state_markers::Opened>,
                    >,
                ) -> $crate::Result<()> {
                    $crate::gpt_clock::start::<$channel, Backend>(gpt, &BACKEND)?;
                    TIMER_QUEUE.initialize(Backend);
                    Ok(())
                }

                /// Dispatch a GPT ISR for this monotonic from its bound interrupt
                /// handler (e.g. an RTIC `#[task(binds = ..)]`). Safe under any
                /// vector wiring; delegates to `RticGptTimerDriver::handle_isr`.
                pub fn handle_isr(which: $crate::gpt::IsrPrototype) {
                    STORAGE.handle_isr(which, Backend::pend)
                }
            }

            impl ::rtic_monotonics::TimerQueueBasedMonotonic for $name {
                type Backend = Backend;
                type Instant = ::rtic_monotonics::fugit::Instant<u64, 1, { $tick_rate_hz }>;
                type Duration = ::rtic_monotonics::fugit::Duration<u64, 1, { $tick_rate_hz }>;
            }
        };
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

    /// Safely dispatch a GPT ISR for the monotonic from its bound interrupt
    /// handler (e.g. an RTIC `#[task(binds = ..)]`).
    ///
    /// Reads the channel's configured IRQ for `which` in a borrow that is
    /// released before dispatch and checks it against the active IRQ; gating on
    /// the IRQ *number* (not the vector contents) works under any vector wiring.
    /// Capture-B (the alarm) takes a lean path; cycle-end and capture-A run the
    /// FSP ISR.
    pub fn handle_isr(&self, which: IsrPrototype, pend_interrupt: impl Fn()) {
        let expected = critical_section::with(|cs| {
            let borrow = self.0.gpt.borrow_ref_mut(cs);
            let gpt = borrow.as_ref()?;
            match which {
                IsrPrototype::Overflow | IsrPrototype::Underflow => gpt.cycle_end_irq(),
                IsrPrototype::CompareA => gpt.capture_a_irq(),
                IsrPrototype::CompareB => gpt.capture_b_irq(),
            }
        });

        // Note: it can be safe if we call the handler in the critical section
        //       and use `start_with_block`, but we want to reduce the critical
        //       section time.

        let Some(expected) = expected else { return };
        if utils::current_irq_get() != Some(expected) {
            return;
        }

        if matches!(which, IsrPrototype::CompareB) {
            // Lean alarm path: clear the ICU IR flag and pend the monotonic
            // queue directly, skipping the FSP capture-B ISR dispatch -- its
            // context lookup, `callback_args` build, trampoline and event match.
            // For a periodic compare match the FSP ISR does no more than this
            // (it never touches GTST); this mirrors `gpt_callback`'s
            // `TIMER_EVENT_COMPARE_B` arm.
            crate::icu::irq_status_clear(expected);
            pend_interrupt();
        } else {
            // SAFETY: the active IRQ is the channel's configured IRQ for `which`.
            //         We can't use `call_fsp_isr_handler` because RTIC introduces
            //         trampolines.
            unsafe { which.call_fsp_isr_handler_unchecked() }
        }
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
