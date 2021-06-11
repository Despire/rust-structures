use super::raw_val_iter::RawValIter;
use super::Vec;

use std::marker::PhantomData;

pub struct Drain<'a, T: 'a> {
    _marker: PhantomData<&'a super::Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> Vec<T> {
    /// Drains the elements from the vector
    /// leaving the underlying allocated memory
    /// untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec::Vec;
    ///
    /// let mut v: Vec<i32> = Vec::new();
    /// v.push(5);
    /// v.push(7);
    ///
    /// {
    ///     let mut iter = v.drain();
    ///     assert_eq!(iter.next().unwrap(), 5);
    ///     assert_eq!(iter.next_back().unwrap(), 7);
    ///     assert_eq!(iter.next(), None);
    /// }
    /// assert_eq!(v.capacity(), 2);
    /// ```
    pub fn drain(&mut self) -> Drain<T> {
        let iter = unsafe { RawValIter::new(&self) };

        // consume all values.
        self.len = 0;

        Drain {
            iter,
            _marker: PhantomData,
        }
    }
}
