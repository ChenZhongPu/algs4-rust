//! # Bottom-up Merge Sort

use std::cmp::PartialOrd;

pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    let mut width = 1;
    let mut aux = a.to_vec();
    let n = a.len();

    while width < n {
        let mut i = 0;
        while i < n {
            let hi = std::cmp::min(i + 2 * width, n);
            let mid = std::cmp::min(i + width, n);

            merge(&a[i..mid], &a[mid..hi], &mut aux[i..hi]);

            a[i..hi].copy_from_slice(&aux[i..hi]);

            i += 2 * width;
        }
        width *= 2;
    }
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
