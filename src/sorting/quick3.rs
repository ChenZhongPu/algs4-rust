//! # Quick Sort 3
//!
//! The implementation from K&R book

use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(a: &mut [T]) {
    if a.len() > 1 {
        qsort(a, 0, a.len() - 1);
    }
}

fn qsort<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    a.swap(lo, (lo + hi) / 2);
    let mut last = lo;

    for i in (lo + 1)..=hi {
        if a[i] < a[lo] {
            last += 1;
            a.swap(last, i);
        }
    }

    a.swap(lo, last);
    qsort(a, lo, last.saturating_sub(1));
    qsort(a, last + 1, hi);
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
