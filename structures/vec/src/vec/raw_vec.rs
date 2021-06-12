use std::alloc;
use std::alloc::handle_alloc_error;
use std::alloc::Layout;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct RawVec<T> {
    buffer: NonNull<T>,
    cap: usize,

    // This marker is necessary for dropck
    // to understand that we logically own a `T`.
    _marker: PhantomData<T>,
}

unsafe impl<T> Send for RawVec<T> where T: Send {}
unsafe impl<T> Sync for RawVec<T> where T: Sync {}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        let cap = if std::mem::size_of::<T>() == 0 { !0 } else { 0 };

        RawVec {
            buffer: NonNull::dangling(),
            cap,
            _marker: PhantomData,
        }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn as_ptr(&self) -> *mut T {
        self.buffer.as_ptr()
    }

    pub fn grow(&mut self) {
        assert!(std::mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // new_cap will never exceed isize::MAX allocations.
            let new_cap = 2 * self.cap;

            // Layout allows a max allocation of usize::MAX.
            // We allow a max allocation of isize::MAX which is
            // half less and therefore this unwrap should not fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();

            (new_cap, new_layout)
        };

        assert!(
            new_cap <= isize::MAX as usize,
            "cannot allocation that amount of memory"
        );

        let raw_ptr = if new_cap == 1 {
            unsafe { alloc::alloc(layout) }
        } else {
            // unwrap shouldn't fail same reason as above.
            let old_layout = Layout::array::<T>(self.cap).unwrap();

            unsafe { alloc::realloc(self.buffer.as_ptr() as *mut u8, old_layout, layout.size()) }
        };

        self.buffer = match NonNull::new(raw_ptr as *mut T) {
            Some(p) => p,
            None => {
                handle_alloc_error(layout);
            }
        };

        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 && std::mem::size_of::<T>() != 0 {
            unsafe {
                // dealocate the underlying buffer.
                alloc::dealloc(
                    self.buffer.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}
