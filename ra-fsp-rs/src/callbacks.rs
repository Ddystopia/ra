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
                "Either `Block` must be `NopBlock` or override `Callback::call`"
            );
        }

        _ = (context, event);

        unimplemented!();
    }

    fn call_with_block(context: &Self, block: core::pin::Pin<&mut Block>, event: Event) {
        _ = block;
        Self::call(context, event)
    }
}

pub(crate) unsafe trait CallbackEvent<E, B: Block = NopBlock>: Copy {
    fn call_fsp_isr_handler(self);
    // Would be safe to alias due to ctrl blocks being in UnsafePinned
    fn context(block: *mut B) -> *mut *const B;
    fn process_args(args: *mut ()) -> (*mut B, *const (), E);
    fn process_static_args(args: *mut ()) -> (*const (), E);
    fn fsp_callback_set<'a>(
        block: Pin<&'a mut B>,
        p_callback: unsafe extern "C" fn(*mut ()),
        p_context: *const core::ffi::c_void,
        user_data: *const (),
    ) -> Result<()>;

    #[inline(always)]
    fn cast_callback<A>(callback: unsafe extern "C" fn(*mut ())) -> unsafe extern "C" fn(*mut A) {
        unsafe { *ptr::from_ref(&callback).cast() }
    }

    #[inline(always)]
    fn callback_set<'a, F: Callback<E, B>>(block: Pin<&mut B>, context: &'a F) -> Result<()> {
        unsafe extern "C" fn trampoline<
            'a,
            E,
            B: Block,
            F: Callback<E, B>,
            D: CallbackEvent<E, B>,
        >(
            args: *mut (),
        ) {
            unsafe {
                let (block, context, event) = D::process_args(args);
                if !block.is_null() {
                    debug_assert!(!context.is_null());

                    let context = &*context.cast::<F>();
                    let this = Pin::new_unchecked(&mut *block);
                    F::call_with_block(context, this, event);
                }
            }
        }

        Self::fsp_callback_set(
            block,
            trampoline::<E, B, F, Self>,
            core::ptr::null(),
            ptr::from_ref(context).cast(),
        )
    }

    #[inline(always)]
    fn callback_set_static<F: Callback<E>>(block: Pin<&mut B>, context: &'static F) -> Result<()> {
        unsafe extern "C" fn trampoline<E, B: Block, F: Callback<E>, D: CallbackEvent<E, B>>(
            args: *mut (),
        ) {
            unsafe {
                let (context, event) = D::process_static_args(args);

                debug_assert!(!context.is_null());

                F::call(&*context.cast::<F>(), event);
            }
        }

        Self::fsp_callback_set(
            block,
            trampoline::<E, B, F, Self>,
            ptr::from_ref(context).cast(),
            core::ptr::null(),
        )
    }

    #[inline(always)]
    fn handle_isr(self, block: Pin<&mut B>) {
        unsafe {
            let this = ptr::from_mut(block.get_unchecked_mut());
            let p_context = Self::context(this);
            // Prevent recursion of `handle_isr`.
            if !(*p_context).is_null() {
                return;
            }

            // Establish provenance, as `Self` isn't in `UnsafePinned`. Callback may be called.
            *p_context = this;

            // isr gets `p_ctrl` -> gets `p_context` with new provenance -> calls trampoline
            // which recovers correct `self` -> calls callback.
            self.call_fsp_isr_handler();

            // Prevent errors if trampoline gets executed out of this function,
            // like user manually calling those isr handles.
            *p_context = ptr::null();
        }
    }
}

impl<F: Fn(Event), Event> Callback<Event> for F {
    fn call(context: &F, event: Event) {
        (context)(event)
    }
}

impl<Event> Callback<Event> for () {
    fn call(_context: &(), _event: Event) {}
}
