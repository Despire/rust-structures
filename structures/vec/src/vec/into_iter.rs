use super::raw_val_iter::RawValIter;
use super::raw_vec::RawVec;
use super::Vec;

use std::mem;
use std::ptr;

/// Construct an interator that will
/// consume the vector by value.
pub struct IntoIter<T> {
    _buffer: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> Vec<T> {
    /// Consumes the vector by value
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// v.push(1);
    ///
    /// let mut iter = v.into_iter();
    /// assert_eq!(iter.next().unwrap(), 1);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        // can't desctruct vec since it's drop
        let iter = unsafe { RawValIter::new(&self) };
        let buffer = unsafe { ptr::read(&self.buffer) };

        // since we will gain ownership of the vector's
        // memory we need to forget it so we avoid
        // double drop.
        mem::forget(self);

        IntoIter {
            _buffer: buffer,
            iter,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
