//! # Quick Sort 2
//!
//! Yet another partition

use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(a: &mut [T]) {
    // optional: random shuffle `a` to eliminate dependence on input
    _sort(a, 0, a.len() - 1);
}

// note `j` can be `0`, so `lo` and `hi` should be `isize`.
// another solution is to check whether `j` is `0`.
fn _sort<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let j = partition(a, lo, hi);
    if j > 0 {
        _sort(a, lo, j - 1);
    }
    _sort(a, j + 1, hi);
}

fn partition<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) -> usize {
    let (mut i, mut j) = (lo, lo);
    while j < hi {
        if a[j] < a[hi] {
            a.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    if a[hi] < a[i] {
        a.swap(i, hi);
    }
    i
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

    #[test]
    fn ascending() {
        let mut v = vec![0, 1, 3, 5, 7];
        sort(&mut v);
        assert_eq!(v, vec![0, 1, 3, 5, 7]);
    }

    #[test]
    fn descending() {
        let mut v = vec![7, 5, 3, 1, 0];
        sort(&mut v);
        assert_eq!(v, vec![0, 1, 3, 5, 7]);
    }
}
