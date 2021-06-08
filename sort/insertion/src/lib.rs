//! # Insertion
//!
//! `insertion` provides sort capabilities using insertion sort
//! using O(n^2) time complexity and O(1) space complexity.

/// Sorts the slice using insertion sort.
///
/// # Examples
///
/// ```
/// use insertion::sort;
///
/// let mut nmbrs = vec![4,3,2,2,1];
/// sort(&mut nmbrs);
///
/// assert_eq!(nmbrs, vec![1,2,2,3,4]);
pub fn sort<T>(slice: &mut [T])
where
    T: Ord,
{
    let mut curr = 1;

    while curr < slice.len() {
        let mut tmp = curr;
        while tmp > 0 && slice[tmp] < slice[tmp - 1] {
            slice.swap(tmp, tmp - 1);
            tmp -= 1;
        }

        curr += 1;
    }
}
