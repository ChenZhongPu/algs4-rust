//! # Merge Sort
//!
//! Divide-and-conquer: sort the left half and right half, and then merge
// use std::cmp::PartialOrd;

pub fn sort<T: Copy + PartialOrd + Default>(a: &mut [T]) {
    merge_sort(a, 0, a.len() - 1);
}

fn merge<T: Copy + PartialOrd + Default>(a: &mut [T], lo: usize, mid: usize, hi: usize) {
    let (n_l, n_r) = (mid - lo + 1, hi - mid);
    let mut left = vec![T::default(); n_l];
    left.copy_from_slice(&a[lo..mid + 1]);
    let mut right = vec![T::default(); n_r];
    right.copy_from_slice(&a[mid + 1..hi + 1]);

    let (mut i, mut j, mut k) = (0, 0, lo);
    while i < n_l && j < n_r {
        if left[i] <= right[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // copy the remainder
    if i < n_l {
        let remain = n_l - i;
        a[k..remain + k].copy_from_slice(&left[i..]);
    }
    if j < n_r {
        let remain = n_r - j;
        a[k..remain + k].copy_from_slice(&right[j..]);
    }
}

fn merge_sort<T: Copy + PartialOrd + Default>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort(a, lo, mid);
    merge_sort(a, mid + 1, hi);
    merge(a, lo, mid, hi);
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
