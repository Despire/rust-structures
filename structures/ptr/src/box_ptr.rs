//! A pointer type for heap allocation.
//!
//! Box provides the simplest for of heap allocation.
//! Boxes provide ownership for the allocation and
//! drop thier contents when they go out of scope.

use std::ptr;
use std::ptr::NonNull;

use std::marker::PhantomData;
use std::marker::Unsize;

use std::alloc;
use std::alloc::handle_alloc_error;
use std::alloc::Layout;

use std::ops::Deref;
use std::ops::DerefMut;

use std::ops::CoerceUnsized;
use std::ops::DispatchFromDyn;

/// A pointer type for heap allocation.
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,

    // This marker is necessary for dropck
    // to understand that we logically own a `T`.
    _marker: PhantomData<T>,
}

// We are Send/Sync if the underlying type is also Send Sync.
unsafe impl<T: ?Sized> Send for Box<T> where T: Send {}
unsafe impl<T: ?Sized> Sync for Box<T> where T: Sync {}

impl<T: Sized> Box<T> {
    /// Allocates memory on the heap and then places `val` into it.
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// let b = Box::new(5);
    /// ```
    pub fn new(val: T) -> Self {
        if std::mem::size_of::<T>() == 0 {
            return Box {
                ptr: NonNull::dangling(),
                _marker: PhantomData,
            }
        }

        // first allocate new memory.
        let val_layout = Layout::new::<T>();
        let raw_ptr = unsafe { alloc::alloc(val_layout) };

        // check if the allocation was successfull.
        let ptr = match NonNull::new(raw_ptr as *mut T) {
            Some(p) => {
                unsafe {
                    ptr::write(p.as_ptr(), val);
                }

                p
            }
            None => {
                handle_alloc_error(val_layout);
            }
        };

        Box {
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        let size = unsafe { std::mem::size_of_val(self.ptr.as_ref()) };

        // only destruct on non-zero types.
        if size != 0 {
            unsafe {
                // destruct the memory.
                ptr::drop_in_place(self.ptr.as_ptr());
                // dealocate the memory.
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::for_value(self.ptr.as_ref()),
                );
            }
        }
    }
}

impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for Box<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<Box<U>> for Box<T> where T: Unsize<U> {}

impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Box<U>> for Box<T> where T: Unsize<U> {}
