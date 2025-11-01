#![cfg_attr(not(test), no_std)]

mod log;
mod macros;
mod pacs;
mod utils;

pub use pin_init;
pub use utils::Irq;

pub mod state_markers {
    use core::any::TypeId;

    pub struct Closed {}
    pub struct Opened {}

    pub const CLOSED_ID: TypeId = core::any::TypeId::of::<Closed>();
    pub const OPENED_ID: TypeId = core::any::TypeId::of::<Opened>();
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

#[cfg(feature = "mod-r_gpt")]
mod gpt_clock;

#[cfg(any(feature = "smoltcp-ether"))]
pub mod smoltcp {
    #[cfg(feature = "smoltcp-ether")]
    pub mod ether;
}
#[cfg(any(feature = "rtic-monotonics-gpt"))]
pub mod rtic {
    #[cfg(any(feature = "rtic-monotonics-gpt"))]
    pub mod gpt;
}

#[cfg(any(feature = "embassy-time-gpt"))]
pub mod embassy {
    #[cfg(any(feature = "embassy-time-gpt"))]
    pub mod gpt;
}

pub use {
    pacs::pac,
    ra_fsp_sys,
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

/// SAFETY: Implementors must ensure that all associated types are zero-initializable,
///         `API` vtable contains valid entries callable with *any* payload and
///         `instance` contains well-formed triplet of pointers to the control
///         block valid for reads and writes for a duration of borrow, a configuration
///         valid for reads for a duration of borrow and the same vtable that is
///         associated to `API` constant.
pub unsafe trait Block {
    type Config: 'static;
    type Instance: 'static;
    type Api: 'static;
    type Context;

    type State;

    const API: &Self::Api;

    fn ctrl(&self) -> *mut core::ffi::c_void;
    fn instance(&self) -> &Self::Instance;
}

// pub unsafe trait ExtensionConf {
//     type CExtConfig: 'static;
//     type Place: 'static;
//
//     fn ext_config() -> Self::CExtConfig;
// }

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
