//! A contiguous growable array type with heap-allocated contents.

mod drain_iter;
mod into_iter;
mod raw_val_iter;
mod raw_vec;

use raw_vec::RawVec;

use std::ptr;

use std::ops::Deref;
use std::ops::DerefMut;

/// A contigious growable array type.
pub struct Vec<T> {
    buffer: RawVec<T>,
    len: usize,
}

unsafe impl<T> Send for Vec<T> where T: Send {}
unsafe impl<T> Sync for Vec<T> where T: Sync {}

impl<T> Vec<T> {
    /// Constructs a new empty vecotr.
    ///
    /// No elemets will be allocated until the elements
    /// are pushed onto the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// ```
    pub fn new() -> Self {
        Vec {
            buffer: RawVec::new(),
            len: 0,
        }
    }

    /// Returns the capacity of the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// v.push(1);
    /// v.push(2);
    /// assert_eq!(v.capacity(), 2);
    pub fn capacity(&self) -> usize {
        self.buffer.cap()
    }

    ///  Pushes the new element as the last item
    ///  on to the vector.
    ///
    ///  # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// v.push(5);
    /// assert_eq!(1, v.len());
    /// ```
    pub fn push(&mut self, elem: T) {
        if self.len == self.capacity() {
            self.buffer.grow()
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    /// Inserts the element at the specified index
    /// shifting all values to the right from the index (idx is included).
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// v.push(5);
    /// v.insert(0, 10);
    /// assert_eq!(v[0], 10);
    /// assert_eq!(v[1], 5);
    /// ```
    pub fn insert(&mut self, idx: usize, elem: T) {
        assert!(idx <= self.len, "index out of bounds");

        if self.capacity() == self.len {
            self.buffer.grow();
        }

        unsafe {
            ptr::copy(self.ptr().add(idx), self.ptr().add(idx + 1), self.len - idx);

            ptr::write(self.ptr().add(idx), elem);
        }

        self.len += 1;
    }

    /// Removes the element at the specified index
    /// shifting all value that are to the left of the index
    /// to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    ///
    /// v.push(5);
    /// v.push(10);
    ///
    /// assert_eq!(v[0], 5);
    ///
    /// v.remove(0);
    ///
    /// assert_eq!(v[0], 10);
    /// ```
    pub fn remove(&mut self, idx: usize) -> T {
        assert!(idx < self.len, "index out of bounds");

        let result = unsafe { ptr::read(self.ptr().add(idx)) };

        unsafe {
            ptr::copy(self.ptr().add(idx + 1), self.ptr().add(idx), self.len - idx);
        }

        self.len -= 1;

        result
    }

    /// Returns the last element from the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// v.push(5);
    /// assert_eq!(5, v.pop().unwrap());
    /// assert_eq!(None, v.pop());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                let result = unsafe { ptr::read(self.ptr().add(self.len)) };

                Some(result)
            }
        }
    }

    fn ptr(&self) -> *mut T {
        self.buffer.as_ptr()
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity() != 0 {
            unsafe {
                // this will call drop on each element.
                ptr::drop_in_place(&mut self[..]);
                // deallocate is handled by rav-vec.
            }
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts_mut(self.buffer.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.buffer.as_ptr(), self.len) }
    }
}
