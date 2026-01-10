use core::{pin::Pin, ptr::NonNull};
use pin_init::InPlaceWrite;
use static_cell::StaticCell;

/**
A trait provinding casting guarantees and type system support. You can think of
this trait as a class of functors over `'a` where `Self` is the `'static` case.

# Safety

It must is always valid to cast `&'static mut MaybeUninit<Self>` to `for<'a, 'b: 'a> &'a mut MaybeUninit<Self::Target<'b>>`.

*/
pub unsafe trait LifetimeDriver {
    type Target<'a>;
}

/**

This is a container indicating an ownership of the driver. This is required to change
the state of the driver as seen in the type system - `Driver::close` will require
`DriverBox<Self>` instead of `Pin<&mut Self>`, though the only difference between
`DriverBox<Self>` and `Pin<&mut Self>` is that `Pin<&mut Self>` can be a result of
some reborrow, while `DriverBox<Self>` guarantees the ownership (as `Pin<Box<Self>>`).

Use [`DriverBox<Driver>::as_mut`] to get `Pin<&mut Driver>`, which can be used to
access the driver methods.

*/
// Invariant: T is fully init, valid, and pinned. So this is basically a `Pin<Box<T>>` without `alloc` crate.
#[must_use = "You probably don't want to drop the driver without handling errors."]
pub struct DriverBox<T>(NonNull<T>);

/// A type used to create a static allocation of the driver.
///
/// Common pattern is to create a `Driver<Closed>` instance, pass it into method
/// like [`DriverPlace::init`] and then call `open` which consumes `DriverBox<Driver<Closed>>`
/// and returns `Result<DriverBox<Driver<Opened>>, DriverBox<Driver<Closed>>>`.
pub struct DriverPlace<T>(StaticCell<T>);

impl<T: LifetimeDriver> DriverBox<T> {
    #[inline]
    #[must_use]
    pub fn new<'a>(driver: &'static mut T) -> DriverBox<T::Target<'a>> {
        DriverBox(NonNull::from_mut(driver).cast())
    }
    #[inline]
    #[must_use]
    pub fn from_pin<'a>(driver: Pin<&'static mut T>) -> DriverBox<T::Target<'a>> {
        // SAFETY: `DriverBox` is equivalent to `Pin<&mut T>`.
        DriverBox(NonNull::from_mut(unsafe { driver.get_unchecked_mut() }).cast())
    }
}

impl<T> DriverBox<T> {
    /// Gets a mutable reference to the pinned value this `DriverBox` points to.
    ///
    /// This method is useful when doing multiple calls to functions that consume the
    /// pinning pointer.
    #[inline]
    #[must_use]
    pub fn as_mut(&mut self) -> Pin<&mut T> {
        // SAFETY: see documentation of `DriverBox`. It is basically the same as `Pin<&mut T>`.
        unsafe { Pin::new_unchecked(self.0.as_mut()) }
    }

}

/*
#[cfg(feature = "alloc")]
impl<T> DriverBox<T> {
    pub fn from_box(driver: Box<T>) -> Self;
    pub fn from_pin_box(driver: Pin<Box<T>>) -> Self;
    pub fn into_box(self) -> Pin<Box<T>>;
}
*/

impl<T: LifetimeDriver> DriverPlace<T> {
    /// Create a new, empty `DriverPlace`.
    ///
    /// It can be initialized at runtime with [`DriverPlace::init()`] or similar methods.
    #[inline]
    #[must_use]
    pub const fn new() -> DriverPlace<T::Target<'static>> {
        DriverPlace(StaticCell::new())
    }

    /// Initialize the `DriverPlace` with a value, returning a `DriverBox` to it.
    ///
    /// Common pattern is to create a `Driver<Closed>` instance, pass it into
    /// this method and then call `open` which consumes `DriverBox<Driver<Closed>>` and
    /// returns `Result<DriverBox<Driver<Opened>>, DriverBox<Driver<Closed>>>`.
    ///
    /// # Panics
    ///
    /// Panics if this `DriverPlace` is already full.
    #[inline]
    #[must_use]
    pub fn init<'a>(&'static self, driver: T) -> DriverBox<T::Target<'a>> {
        let this = self.0.init(driver);
        DriverBox::new(this)
    }

    /// Try initializing the `DriverPlace` with a value, returning a `DriverBox` to it.
    ///
    /// If this `DriverPlace` is already full, it returns `None`.
    ///
    /// Will only return a `Some(DriverBox)` when the `DriverPlace` was not yet initialized.
    #[inline]
    #[must_use]
    pub fn try_init<'a>(&'static self, driver: T) -> Option<DriverBox<T::Target<'a>>> {
        let this = self.0.try_init(driver)?;
        Some(DriverBox::new(this))
    }

    /// Use the given pin-initializer to write a value into `self`.
    ///
    /// Does not drop the current value and considers it as uninitialized memory.
    ///
    /// If the initializer fails, `DriverPlace` still becomes full.
    ///
    /// # Panics
    ///
    /// Panics if this `DriverPlace` is already full.
    #[inline]
    #[must_use]
    pub fn write_pin_init<'a, E>(
        &'static self,
        init: impl pin_init::PinInit<T, E>,
    ) -> Result<DriverBox<T::Target<'a>>, E> {
        let pin = self.0.uninit().write_pin_init(init)?;
        Ok(DriverBox::from_pin(pin))
    }

    /// Use the given pin-initializer to try to write a value into `self`.
    ///
    /// Does not drop the current value and considers it as uninitialized memory.
    ///
    /// If this `DriverPlace` is already full, it returns `None`.
    /// If the initializer fails, `DriverPlace` still becomes full.
    #[inline]
    #[must_use]
    pub fn try_write_pin_init<'a, E>(
        &'static self,
        init: impl pin_init::PinInit<T, E>,
    ) -> Option<Result<DriverBox<T::Target<'a>>, E>> {
        let pin = self.0.try_uninit()?.write_pin_init(init);
        Some(pin.map(DriverBox::from_pin))
    }
}
