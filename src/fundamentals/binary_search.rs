//! # Binary searching
//!
//! Binary searching for an integer in a sorted array of integers.
//!
//! The `index_of` operations takes logarithmic time in the worst case.
use std::cmp::Ordering;
/// Returns the index of an integer.
/// None if not found.
///
pub fn index_of(key: i32, a: &[i32]) -> Option<usize> {
    let mut lo = 0;
    let mut hi = (a.len() - 1) as i32;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        match key.cmp(&a[mid as usize]) {
            Ordering::Less => hi = mid - 1,
            Ordering::Greater => lo = mid + 1,
            Ordering::Equal => return Some(mid as usize),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [1, 2, 5, 6, 10, 20, 30, 31];
        assert_eq!(index_of(2, &a), Some(1));
        assert_eq!(index_of(31, &a), Some(7));
        assert_eq!(index_of(7, &a), None);
    }
}
