//! Thread-safe reference-counting pointers.
//!
//! `Arc<T>` provides shared ownership of the value `T`
//! allocated on the heap. Invoking `clone` produces
//! a new pointer to the same allocation on the heap.
//!
//! Arc disallows mutation by default. Arc represents only
//! immutable references to the owned data `T`.

use std::sync::atomic;
use std::sync::atomic::Ordering;

use std::alloc;
use std::alloc::handle_alloc_error;
use std::alloc::Layout;

use std::ptr;
use std::ptr::NonNull;

use std::marker;
use std::marker::PhantomData;
use std::marker::Unsize;

use std::ops::CoerceUnsized;
use std::ops::Deref;
use std::ops::DispatchFromDyn;

struct ArcBox<T: ?Sized> {
    rc: atomic::AtomicUsize,
    data: T,
}

/// A thread-safe reference-cointing pointer.
pub struct Arc<T: ?Sized> {
    ptr: NonNull<ArcBox<T>>,

    // This marker is necessary for dropck
    // to understand that we logically own a `T`.
    _marker: PhantomData<T>,
}

// Opt in for multi-threading only if the underlying type is also thread-safe.
unsafe impl<T: ?Sized> marker::Send for Arc<T> where T: Send + Sync {}
unsafe impl<T: ?Sized> marker::Sync for Arc<T> where T: Send + Sync {}

impl<T: Sized> Arc<T> {
    /// Constructs a new Arc<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use ptr::Arc;
    ///
    /// let five = Arc::new(5);
    /// ```
    pub fn new(val: T) -> Self {
        // allocate memory for the inner
        // ArcBox on the heap to which
        // all cloned instances will point to.
        let arc_box_layout = Layout::new::<ArcBox<T>>();
        let raw_ptr = unsafe { alloc::alloc(arc_box_layout) };

        let ptr = match NonNull::new(raw_ptr as *mut ArcBox<T>) {
            Some(p) => {
                let arc_box = ArcBox {
                    rc: atomic::AtomicUsize::new(1),
                    data: val,
                };

                unsafe {
                    ptr::write(p.as_ptr(), arc_box);
                }

                p
            }
            None => {
                handle_alloc_error(arc_box_layout);
            }
        };

        Arc {
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    /// Makes a clone of the `Arc` pointer.
    ///
    /// This creates another pointer to the same allocated
    /// data on the heap and increases the reference count.
    ///
    /// # Examples
    ///
    /// ```
    /// use ptr::Arc;
    ///
    /// let five = Arc::new(5);
    /// let _ = Arc::clone(&five);
    /// ```
    fn clone(&self) -> Arc<T> {
        let arc_box = unsafe { self.ptr.as_ref() };

        let old = arc_box.rc.fetch_add(1, Ordering::Relaxed);

        if old == 0 || old == isize::MAX as usize {
            std::process::abort();
        }

        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Drop for Arc<T> {
    fn drop(&mut self) {
        let arc_inner = unsafe { self.ptr.as_ref() };
        let old = arc_inner.rc.fetch_sub(1, Ordering::Release);

        if old != 1 {
            return; // we drop only on count 0
        }

        // prevent reordering of the use and
        // deletion of the data.
        atomic::fence(Ordering::Acquire);

        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());

            alloc::dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::for_value(self.ptr.as_ref()),
            );
        }
    }
}

impl<T: ?Sized> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let arc_box = unsafe { self.ptr.as_ref() };
        &arc_box.data
    }
}

impl<T: ?Sized> AsRef<T> for Arc<T> {
    fn as_ref(&self) -> &T {
        let arc_box = unsafe { self.ptr.as_ref() };
        &arc_box.data
    }
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<Arc<U>> for Arc<T> where T: Unsize<U> {}
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Arc<U>> for Arc<T> where T: Unsize<U> {}
