//! # Merge
//!
//! `merge` provides sort capabilities using merge sort
//! using O(n log(n)) time complexity and O(h) space complexity.

/// Sorts the slice withing the specified interval [b, e)
/// using recursive merge sort.
fn internal_mergesort<T>(b: usize, e: usize, slice: &mut [T])
where
    T: Ord,
{
    if e - b > 1 {
        let mid = b + (e - b) / 2;
        internal_mergesort(b, mid, slice);
        internal_mergesort(mid, e, slice);
        internal_merge(mid, b, e, slice);
    }
}

/// Merges two sorted subslices into one sorted slice.
///
/// The two subslices are seperated by indices `b, e, mid`
/// where `b` is the beginning of the first sublisce
/// `mid` is the end of the first subslice and at the
/// same time the beginning of the second sublice.
/// `e` is the end of the second subslice.
///
/// The merge is done in place. Only constant memory is allocated.
fn internal_merge<T>(mid: usize, b: usize, e: usize, slice: &mut [T])
where
    T: Ord,
{
    let mut left_begin = b;
    let mut left_end = mid;
    let mut right_begin = mid;
    let right_end = e;

    loop {
        if left_begin == left_end || right_begin == right_end {
            break;
        }

        if slice[left_begin] <= slice[right_begin] {
            left_begin += 1;
        } else {
            (&mut slice[left_begin..right_begin + 1]).rotate_right(1);
            left_begin += 1;
            left_end += 1;
            right_begin += 1;
        }
    }
}

/// Sorts the slice using recursive merge sort.
///
/// # Examples
///
/// ```
/// use merge::sort;
///
/// let mut nmbrs = vec![4,3,2,5,1];
/// sort(&mut nmbrs);
///
/// assert_eq!(nmbrs, vec![1,2,3,4,5]);
pub fn sort<T>(slice: &mut [T])
where
    T: Ord,
{
    internal_mergesort(0, slice.len(), slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_merge() {
        let mut nmbrs = vec![1, 5, 2, 3, 4];
        internal_merge(2, 0, nmbrs.len(), &mut nmbrs);
        assert_eq!(nmbrs, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_internal_mergesort() {
        let mut nmbrs = vec![1, 5, 2, 3, 4];
        internal_mergesort(0, nmbrs.len(), &mut nmbrs);
        assert_eq!(nmbrs, vec![1, 2, 3, 4, 5]);
    }
}
