#![cfg_attr(not(test), no_std)]

pub use ra_fsp_sys;

#[cfg(feature = "ra0e1")]
pub use ra0e1_pac as pac;
#[cfg(feature = "ra2a1")]
pub use ra2a1_pac as pac;
#[cfg(feature = "ra2a2")]
pub use ra2a2_pac as pac;
#[cfg(feature = "ra2e1")]
pub use ra2e1_pac as pac;
#[cfg(feature = "ra2e2")]
pub use ra2e2_pac as pac;
#[cfg(feature = "ra2e3")]
pub use ra2e3_pac as pac;
#[cfg(feature = "ra2l1")]
pub use ra2l1_pac as pac;
#[cfg(feature = "ra4e1")]
pub use ra4e1_pac as pac;
#[cfg(feature = "ra4e2")]
pub use ra4e2_pac as pac;
#[cfg(feature = "ra4m1")]
pub use ra4m1_pac as pac;
#[cfg(feature = "ra4m2")]
pub use ra4m2_pac as pac;
#[cfg(feature = "ra4m3")]
pub use ra4m3_pac as pac;
#[cfg(feature = "ra4t1")]
pub use ra4t1_pac as pac;
#[cfg(feature = "ra4w1")]
pub use ra4w1_pac as pac;
#[cfg(feature = "ra6e1")]
pub use ra6e1_pac as pac;
#[cfg(feature = "ra6e2")]
pub use ra6e2_pac as pac;
#[cfg(feature = "ra6m1")]
pub use ra6m1_pac as pac;
#[cfg(feature = "ra6m2")]
pub use ra6m2_pac as pac;
#[cfg(feature = "ra6m3")]
pub use ra6m3_pac as pac;
#[cfg(feature = "ra6m4")]
pub use ra6m4_pac as pac;
#[cfg(feature = "ra6m5")]
pub use ra6m5_pac as pac;
#[cfg(feature = "ra6t1")]
pub use ra6t1_pac as pac;
#[cfg(feature = "ra6t2")]
pub use ra6t2_pac as pac;
#[cfg(feature = "ra6t3")]
pub use ra6t3_pac as pac;
#[cfg(feature = "ra8m1")]
pub use ra8m1_pac as pac;
#[cfg(feature = "ra8d1")]
pub use ra8d1_pac as pac;
#[cfg(feature = "ra8t1")]
pub use ra8t1_pac as pac;

pub use ra_fsp_sys::generated::e_elc_event;

#[cfg(feature = "mod-r_ether")]
pub mod ether;
#[cfg(feature = "mod-r_ether_phy")]
pub mod ether_phy;
#[cfg(feature = "mod-r_ioport")]
pub mod ioport;

mod macros;

#[doc(hidden)]
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
        pub const fn raw_get(this: *const Self) -> *mut T {
            unsafe { UnsafeCell::raw_get(&raw const (*this).1) }
        }
    }
}

