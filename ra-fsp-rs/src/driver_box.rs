use crate::pin_init;
use core::{fmt::Debug, ops::Deref, pin::Pin, ptr::NonNull};
use static_cell::StaticCell;

/**
A trait providing casting guarantees and type system support. You can think of
this trait as a class of functors over `'a` where `Self` is the `'static` case.

# Safety

It must is always be valid to cast `&'static mut MaybeUninit<Self>` to `for<'a, 'b: 'a> &'a mut MaybeUninit<Self::Target<'b>>`.

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

unsafe impl<T: Send> Send for DriverBox<T> {}
unsafe impl<T: Sync> Sync for DriverBox<T> {}

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
    pub fn as_ref(&self) -> Pin<&T> {
        // SAFETY: the same as `Pin::get_ref`.
        unsafe { Pin::new_unchecked(self.0.as_ref()) }
    }
    pub fn leak<'a>(self) -> Pin<&'a mut T> {
        let mut this = core::mem::ManuallyDrop::new(self);
        // SAFETY: `self` is pinned thus creating a pin is fine. Memory where 
        //          we point is leaked and will always be available for reads and writes.
        unsafe { Pin::new_unchecked(this.0.as_mut()) }
    }
    pub fn forget(self) {
        core::mem::forget(self)
    }
    /// Refer to [`Pin::get_unchecked_mut`].
    pub unsafe fn get_unchecked_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
    /// # Safety
    ///
    /// `DriverBox` behaves as `Pin<Box<T>>`, so beyond [`Pin::new_unchecked`]'s
    /// pinning contract the caller must uphold ownership semantics:
    ///
    /// - ownership transfers to the box — dropping it runs `T`'s drop in
    ///   place, so the caller must not drop, move out of, or otherwise reuse
    ///   the value;
    /// - [`DriverBox::leak`] mints a borrow of any caller-chosen lifetime
    ///   (including `'static`), so the backing memory must never be
    ///   deallocated.
    pub unsafe fn new_unchecked(driver: &mut T) -> Self {
        DriverBox(NonNull::from_mut(driver))
    }
}

impl<T> Deref for DriverBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        Pin::get_ref(Self::as_ref(self))
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
    pub const fn new() -> DriverPlace<T> {
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
        // TODO: replace to this (v) when `pin_init` is updated.
        // use pin_init::InPlaceWrite;
        // let pin = self.0.uninit().write_pin_init(init)?;
        // Ok(DriverBox::from_pin(pin))
        let uninit = self.0.uninit();
        unsafe { init.__pinned_init(uninit.as_mut_ptr())? };
        let pin = Pin::static_mut(unsafe { uninit.assume_init_mut() });
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
        // TODO: replace to this (v) when `pin_init` is updated.
        // use pin_init::InPlaceWrite;
        // let pin = self.0.try_uninit()?.write_pin_init(init);
        // Some(pin.map(DriverBox::from_pin))
        let uninit = self.0.try_uninit()?;
        if let Err(e) = unsafe { init.__pinned_init(uninit.as_mut_ptr()) } {
            return Some(Err(e));
        }
        let pin = Pin::static_mut(unsafe { uninit.assume_init_mut() });
        Some(Ok(DriverBox::from_pin(pin)))
    }
}

impl<T: Debug> Debug for DriverBox<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value: &T = &self;
        f.debug_tuple("DriverBox").field(value).finish()
    }
}

impl<T> Drop for DriverBox<T> {
    fn drop(&mut self) {
        // SAFETY: `self` is equivalent to `Pin<Box<T>>`, thus it is correct to drop it like that.
        unsafe { core::ptr::drop_in_place(self.0.as_ptr()) }
    }
}
