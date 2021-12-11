//! # Merge Sort Using Slice
//!
//! Divide-and-conquer: sort the left half and right half, and then merge
use std::cmp::PartialOrd;

pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    let mid = a.len() / 2;
    if mid == 0 {
        return;
    }

    sort(&mut a[..mid]);
    sort(&mut a[mid..]);

    let mut aux = a.to_vec();
    merge(&a[..mid], &a[mid..], &mut aux[..]);

    // copy back to the original array
    a.copy_from_slice(&aux);
}

fn merge<T: Copy + PartialOrd>(left: &[T], right: &[T], aux: &mut [T]) {
    // head of left, head of right, and index
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            aux[k] = left[i];
            k += 1;
            i += 1;
        } else {
            aux[k] = right[j];
            k += 1;
            j += 1;
        }
    }

    // copy the rest to returned aux
    if i < left.len() {
        aux[k..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        aux[k..].copy_from_slice(&right[j..]);
    }
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
