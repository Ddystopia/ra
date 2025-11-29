#![cfg_attr(not(test), no_std)]

/*

Rule for callbacks:
- if interrupt callback can be preempted, it must have `&` for the block for
  `p_context` to remain good.

*/

mod callbacks;
mod log;
mod macros;
mod pacs;
pub mod utils;

pub mod state_markers {
    pub struct Closed {}
    pub struct Opened {}
}

#[cfg(feature = "mod-r_display_api")]
pub mod display_api;
#[cfg(feature = "mod-r_ether")]
pub mod ether;
#[cfg(feature = "mod-r_ether_phy")]
pub mod ether_phy;
#[cfg(feature = "mod-r_glcdc")]
pub mod glcdc;
#[cfg(feature = "mod-r_gpt")]
pub mod gpt;
#[cfg(feature = "mod-r_ioport")]
pub mod ioport;
#[cfg(feature = "mod-r_timer_api")]
pub mod timer_api;

pub mod systick {
    pub fn system_core_clock(_: critical_section::CriticalSection<'_>) -> u32 {
        unsafe { ra_fsp_sys::SystemCoreClock }
    }
}

#[cfg(any(feature = "embassy-time-gpt", feature = "rtic-monotonics-gpt"))]
mod gpt_clock;

#[cfg(any(feature = "smoltcp-ether"))]
pub mod smoltcp {
    #[cfg(feature = "smoltcp-ether")]
    pub mod ether;
}
#[cfg(any(feature = "rtic-monotonics-gpt"))]
pub mod rtic {
    #[cfg(feature = "rtic-monotonics-gpt")]
    pub mod gpt;
}

#[cfg(any(feature = "embassy-time-gpt"))]
pub mod embassy {
    #[cfg(feature = "embassy-time-gpt")]
    pub mod gpt;
}

#[cfg(any(feature = "embedded-graphics-glcdc"))]
pub mod embedded_graphics {
    #[cfg(feature = "embedded-graphics-glcdc")]
    pub mod glcdc;
}

pub use {
    callbacks::{Callback, NopBlock},
    cortex_m,
    pacs::pac,
    pin_init, ra_fsp_sys, ra_fsp_sys as sys,
    ra_fsp_sys::generated::{e_elc_event, e_fsp_err, fsp_err_t},
};

pub type Result<T> = core::result::Result<T, fsp_err_t>;

pub const FSP_VERSION: (u32, u32, u32) = (
    ra_fsp_sys::generated::FSP_VERSION_MAJOR,
    ra_fsp_sys::generated::FSP_VERSION_MINOR,
    ra_fsp_sys::generated::FSP_VERSION_PATCH,
);

pub unsafe trait DynBlock<API: 'static> {
    fn c_api(&self) -> &'static API;
}

/// # SAFETY
/// - `API` vtable is exactly the same as provided by FSP for a given HAL block.
/// - `ctrl` returns a writable pointer to the ctrl block, valid until the `Block` is dropped.
/// - `instance` retuns a reference to FSP instance with pointers valid as in previous requirement.
/// - If `Self` is ZST, align must not be equal to 16384 except for `NopBlock`.
pub unsafe trait Block: Sized {
    type Config: 'static;
    type Instance: 'static;
    type Api: 'static;

    type State;

    const API: &Self::Api;

    fn ctrl(&self) -> *mut core::ffi::c_void;
    fn instance(&self) -> &Self::Instance;
}

unsafe impl<T: Block> DynBlock<T::Api> for T {
    fn c_api(&self) -> &'static T::Api {
        T::API
    }
}

// Wait for UnsafePinned to be stabilized in core
// https://github.com/rust-lang/rust/issues/125735

#[doc(hidden)]
#[allow(dead_code)]
mod unsafe_pinned {
    use core::{cell::UnsafeCell, marker::PhantomPinned};

    pub struct UnsafePinned<T: ?Sized>(PhantomPinned, UnsafeCell<T>);

    unsafe impl<T: ?Sized + Sync> Sync for UnsafePinned<T> {}
    unsafe impl<T: ?Sized + Send> Send for UnsafePinned<T> {}

    impl<T> UnsafePinned<T> {
        pub const fn new(value: T) -> Self {
            Self(PhantomPinned, UnsafeCell::new(value))
        }
    }

    impl<T: ?Sized> UnsafePinned<T> {
        pub const fn get(&self) -> *mut T {
            self.1.get()
        }
        pub const fn get_mut(&mut self) -> &mut T {
            self.1.get_mut()
        }
        pub const fn raw_get(this: *const Self) -> *mut T {
            unsafe { UnsafeCell::raw_get(&raw const (*this).1) }
        }
    }
}
