#![cfg_attr(not(test), no_std)]

mod log;
mod macros;
mod pacs;

#[cfg(feature = "mod-r_ether")]
pub mod ether;
#[cfg(feature = "mod-r_ether_phy")]
pub mod ether_phy;
#[cfg(feature = "mod-r_gpt")]
pub mod gpt;
#[cfg(feature = "mod-r_ioport")]
pub mod ioport;
#[cfg(feature = "mod-r_timer_api")]
pub mod timer_api;

#[cfg(any(feature = "smoltcp-ether"))]
pub mod smoltcp {
    #[cfg(feature = "smoltcp-ether")]
    pub mod ether;
}
#[cfg(any(feature = "rtic-monotonic-gpt"))]
pub mod rtic {
    #[cfg(any(feature = "rtic-monotonic-gpt"))]
    pub mod gpt;
}

use core::pin::Pin;
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

pub unsafe trait Block {
    type CConfig: 'static;
    type CInstance: 'static;
    type CApi: 'static;

    const API: &Self::CApi;

    fn instance(self: Pin<&mut Self>) -> &Self::CInstance;
}

unsafe impl<T: Block> DynBlock<T::CApi> for T {
    fn c_api(&self) -> &'static T::CApi {
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
