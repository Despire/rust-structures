//! # Bubble
//!
//! `bubble` provides sort capabilities using bubble sort
//! using O(n^2) time complexity and O(1) space complexity.

/// Sorts the slice using bubble sort.
///
/// # Examples
///
/// ```
/// use bubble::sort;
///
/// let mut nmbrs = vec![4,3,2,2,1];
/// sort(&mut nmbrs);
///
/// assert_eq!(nmbrs, vec![1,2,2,3,4]);
pub fn sort<T>(slice: &mut [T])
where
    T: Ord,
{
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..slice.len() {
            if slice[i] < slice[i - 1] {
                swapped = true;
                slice.swap(i, i - 1);
            }
        }
    }
}
