use core::{pin::Pin, ptr};

use crate::{Block, Result};

#[repr(align(16384))]
pub struct NopBlock;

/// SAFETY: This is a special case outlined in `Block`'s doc
unsafe impl Block for NopBlock {
    type Config = ();
    type Instance = ();
    type Api = ();
    type State = ();
    const API: &Self::Api = &();

    fn ctrl(&self) -> *mut core::ffi::c_void {
        unimplemented!()
    }

    fn instance(&self) -> &Self::Instance {
        unimplemented!()
    }
}

pub trait Callback<Event, Block: crate::Block = NopBlock> {
    fn call(context: &Self, event: Event) {
        // Intent is that if user implemented `call_with_block` is unless they call
        // this method, const assert won't blow up in their face as it is lazy.

        const {
            assert!(
                size_of::<Block>() == 0 && align_of::<Block>() == 16384,
                "You can't call `call`, use `call_with_block`. Either `Block` must be `NopBlock` or override `Callback::call`"
            );
        }

        _ = (context, event);

        unimplemented!();
    }

    fn call_with_block(context: &Self, _block: Pin<&mut Block>, event: Event) {
        Self::call(context, event)
    }
}

#[allow(unused)] // When no drivers are enabled
pub(crate) unsafe trait CallbackEvent<E>: Block {
    // Would be safe to alias due to ctrl blocks being in UnsafePinned
    fn context(this: *mut Self) -> *mut *const Self;
    fn process_args(args: *mut ()) -> (*mut Self, *const (), E);
    fn process_static_args(args: *mut ()) -> (*const (), E);
    fn user_data(this: *mut Self) -> *const ();
    fn fsp_callback_set<'a>(
        self: Pin<&'a mut Self>,
        p_callback: unsafe extern "C" fn(*mut ()),
        p_context: *const core::ffi::c_void,
        user_data: *const (),
    ) -> Result<()>;

    #[inline(always)]
    fn cast_callback<A>(callback: unsafe extern "C" fn(*mut ())) -> unsafe extern "C" fn(*mut A) {
        unsafe { *ptr::from_ref(&callback).cast() }
    }

    #[inline(always)]
    fn callback_set<'a, F: Callback<E, Self>>(self: Pin<&mut Self>, context: &'a F) -> Result<()> {
        unsafe extern "C" fn trampoline<'a, E, B: CallbackEvent<E>, F: Callback<E, B>>(
            args: *mut (),
        ) {
            unsafe {
                let (this, context, event) = B::process_args(args);
                if !this.is_null() {
                    debug_assert!(!context.is_null());

                    let context = &*context.cast::<F>();
                    let this = Pin::new_unchecked(&mut *this);
                    F::call_with_block(context, this, event);
                }
            }
        }

        Self::fsp_callback_set(
            self,
            trampoline::<E, Self, F>,
            core::ptr::null(),
            ptr::from_ref(context).cast(),
        )
    }

    #[inline(always)]
    fn callback_set_static<F: Callback<E>>(
        self: Pin<&mut Self>,
        context: &'static F,
    ) -> Result<()> {
        unsafe extern "C" fn trampoline<E, B: CallbackEvent<E>, F: Callback<E>>(args: *mut ()) {
            unsafe {
                let (context, event) = B::process_static_args(args);

                debug_assert!(!context.is_null());

                F::call(&*context.cast::<F>(), event);
            }
        }

        Self::fsp_callback_set(
            self,
            trampoline::<E, Self, F>,
            ptr::from_ref(context).cast(),
            core::ptr::null(),
        )
    }

    #[inline(always)]
    fn with_callback_provenance<R>(self: Pin<&mut Self>, f: impl FnOnce() -> R) -> R {
        unsafe {
            let this = ptr::from_mut(self.get_unchecked_mut());

            // When user_data is null, either no callback or a static callback is registered.
            // In the static case, p_context holds `&'static F` for the static trampoline and
            // must not be clobbered. In the no-callback case FSP's p_callback is null so
            // nothing reads p_context. Only touch p_context for non-static callbacks.
            if Self::user_data(this).is_null() {
                return f();
            }

            let p_context = Self::context(this);
            let prev_context = *p_context;

            // Establish provenance, as `Self` isn't in `UnsafePinned`. Callback may be called.
            *p_context = this;

            // isr or something gets `p_ctrl` -> gets `p_context` with new provenance -> calls trampoline
            // which recovers correct `self` -> calls callback.
            let res = f();

            // Prevent errors if trampoline gets executed out of this function,
            // like user manually calling those isr handles. Restore previous provenance.
            *p_context = prev_context;

            res
        }
    }
}

impl<F: Fn(Pin<&mut B>, Event), Event, B: Block> Callback<Event, B> for F {
    fn call_with_block(context: &Self, block: Pin<&mut B>, event: Event) {
        (context)(block, event)
    }
}

impl<Event, B: Block> Callback<Event, B> for fn(Event) {
    fn call(context: &Self, event: Event) {
        (context)(event)
    }
}

impl<Event> Callback<Event> for () {
    fn call(_context: &(), _event: Event) {}
}
