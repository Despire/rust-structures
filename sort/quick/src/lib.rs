//! # Quick
//!
//! `quick` provides sort capabilities using quick sort
//! using O(n log(n)) on average and O(n^2) at worst time complexity
//! and O(h) space complexity.

/// Sorts the slice within the specified intervar [b, e)
/// using recursive quick sort.
fn internal_quicksort<T>(b: usize, e: usize, slice: &mut [T])
where
    T: Ord,
{
    if e - b > 1 {
        let p = b + (e - b) / 2;
        let partition_index = internal_partition(p, b, e, slice);
        internal_quicksort(b, partition_index, slice);
        internal_quicksort(partition_index + 1, e, slice);
    }
}

/// Partition the slice based on the pivot at index `p`,
/// such that all the elements that are <= to the pivot are
/// to the left of it and all elements that are > are to the right.
///
/// After the partition the pivot will be at the correct possition and
/// within the specified partition range.
///
/// Indices `b` and `e` denotes the subslice which should be partitioned.
///
/// The partition is done in place. Only constant memory is allocated.
fn internal_partition<T>(mut p: usize, mut b: usize, mut e: usize, slice: &mut [T]) -> usize
where
    T: Ord,
{
    slice.swap(p, b);

    p = b;
    b += 1;
    e -= 1;

    let partition_index = loop {
        if e < b {
            break b;
        }

        if &slice[b] <= &slice[p] {
            b += 1;
        } else {
            slice.swap(e, b);
            e -= 1;
        }
    };

    slice.swap(p, partition_index - 1);
    partition_index - 1
}

/// Sorts the slice using recursive quick sort.
///
/// # Examples
///
/// ```
/// use quick::sort;
///
/// let mut nmbrs = vec![4,3,2,5,1];
/// sort(&mut nmbrs);
///
/// assert_eq!(nmbrs, vec![1,2,3,4,5]);
pub fn sort<T>(slice: &mut [T])
where
    T: Ord,
{
    internal_quicksort(0, slice.len(), slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_quicksort() {
        let mut nmbrs = vec![1, 5, 2, 3, 4];
        internal_quicksort(0, nmbrs.len(), &mut nmbrs);
        assert_eq!(nmbrs, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_internal_partition() {
        let mut nmbrs = vec![1, 5, 2, 3, 4];
        internal_partition(3, 0, nmbrs.len(), &mut nmbrs);
        assert_eq!(nmbrs, vec![2, 1, 3, 4, 5]);
    }
}
