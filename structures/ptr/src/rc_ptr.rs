//! Single-threaded reference counting pointers.
//!
//! Rc<T> provides shared ownership of the value `T`
//! allocated in the heap. Invoking `clone` produces
//! a new pointer to the same allocation in the heap.
//!
//! Rc disallows mutation by default. Rc represents only
//! immutable reference to the owned data `T`.
use std::ptr;
use std::ptr::NonNull;

use std::alloc;
use std::alloc::Layout;
use std::alloc::handle_alloc_error;

use std::marker;
use std::marker::Unsize;
use std::marker::PhantomData;

use std::ops::CoerceUnsized;
use std::ops::DispatchFromDyn;

use std::ops::Deref;

struct RcBox<T: ?Sized> {
    rc: usize,
    data: T,
}

/// A single-threaded reference-counting pointer.
pub struct Rc<T: ?Sized> {
    ptr: NonNull<RcBox<T>>,

    // This marker is necessary for dropck
    // to understand that we logically own a `T`.
    _marker: PhantomData<T>,
}

// Opt out of Send/Sync
impl<T: ?Sized> !marker::Send for Rc<T> {}
impl<T: ?Sized> !marker::Sync for Rc<T> {}

impl<T: Sized> Rc<T> {
    /// Constructs a new Rc<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use ptr::Rc;
    ///
    /// let five = Rc::new(5);
    /// ```
    pub fn new(val: T) -> Self {
        // allocate memory for the inner
        // RcBox on the heap to which
        // all instances will point to.
        let rc_box_layout = Layout::new::<RcBox<T>>();
        let raw_ptr = unsafe { alloc::alloc(rc_box_layout) };

        let ptr = match NonNull::new(raw_ptr as *mut RcBox<T>) {
            Some(p) => {
                let rc_box = RcBox {
                    rc: 1 as usize,
                    data: val,
                };

                unsafe {
                    ptr::write(p.as_ptr(), rc_box);
                }

                p
            }
            None => {
                handle_alloc_error(rc_box_layout);
            }
        };

        Rc {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Gets the number of Rc pointing to the
    /// allocated heap memory of `T`
    /// 
    /// # Examples
    ///
    /// ```
    /// use ptr::Rc;
    ///
    /// let five = Rc::new(5);
    /// let _second = Rc::clone(&five);
    ///
    /// assert_eq!(2, Rc::strong_count(&five));
    /// ```
    pub fn strong_count(&self) -> usize {
        unsafe {
            self.ptr.as_ref().rc
        }
    }
}

impl<T: ?Sized> Clone for Rc<T> {
    /// Makes a clone of the `Rc` pointer.
    ///
    /// This creates another pointer to the same allocated
    /// data on the heap and increases the reference count.
    ///
    /// # Examples
    ///
    /// ```
    /// use ptr::Rc;
    /// let five = Rc::new(5);
    /// let _ = Rc::clone(&five);
    /// ```
    fn clone(&self) -> Rc<T> {
        // increment count.
        let rc = unsafe { self.ptr.as_ref().rc };

        // abort on overflow.
        if rc == 0 || rc == usize::MAX {
            panic!("overflow of reference count");
        }

        // increment ref count.
        unsafe {
            (*self.ptr.as_ptr()).rc += 1;
        }

        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Drop for Rc<T> {
    fn drop(&mut self) {
        let count = unsafe {
            self.ptr.as_mut().rc -= 1;
            self.ptr.as_ref().rc
        };

        if count == 0 {
            unsafe {
                // destruct the RcBox<T>
                // along with the counter and the drop of T
                // will be called.
                ptr::drop_in_place(self.ptr.as_ptr());

                // dealloc the memory.
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::for_value(self.ptr.as_ref()),
                );
            }
        }
    }
}

impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let rc_box = unsafe { self.ptr.as_ref() };
        &rc_box.data
    }
}

impl<T: ?Sized> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        let rc_box = unsafe { self.ptr.as_ref() };
        &rc_box.data
    }
}


impl<T: ?Sized, U: ?Sized> CoerceUnsized<Rc<U>> for Rc<T> where T: Unsize<U> {}
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Rc<U>> for Rc<T> where T: Unsize<U> {}
