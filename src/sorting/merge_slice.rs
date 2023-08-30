//! # Merge Sort Using Slice
//!
//! Divide-and-conquer: sort the left half and right half, and then merge
use std::cmp::PartialOrd;
pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    let mid = a.len() / 2;
    if mid == 0 {
        return;
    }
    let (left, right) = a.split_at_mut(mid);
    sort(left);
    sort(right);
    let result = merge(left, right);
    a.copy_from_slice(&result);
}

// Alternatively, we can pass a slice to the merge function as the result
fn merge<T: Copy + PartialOrd>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    if i < left.len() {
        result.extend_from_slice(&left[i..]);
    }
    if j < right.len() {
        result.extend_from_slice(&right[j..]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_int() {
        let mut v = vec![6, 2, 8, 1, 0, 9];
        sort(&mut v);
        assert_eq!(v, vec![0, 1, 2, 6, 8, 9]);
    }

    #[test]
    fn sort_char() {
        let mut v = vec!['S', 'O', 'R', 'T', 'E', 'X', 'A', 'M', 'P', 'L', 'E'];
        sort(&mut v);
        assert_eq!(
            v,
            vec!['A', 'E', 'E', 'L', 'M', 'O', 'P', 'R', 'S', 'T', 'X']
        )
    }
}
