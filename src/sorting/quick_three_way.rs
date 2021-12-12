//! # Quicksort with 3-way partitioning

use std::cmp::PartialOrd;

pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    // optional: random shuffle `a` to eliminate dependence on input
    _sort(a, 0, a.len() - 1);
}

fn _sort<T: Copy + PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let (mut lt, mut i, mut gt) = (lo, lo + 1, hi);
    let v = a[lo];
    while i <= gt {
        if a[i] < v {
            a.swap(lt, i);
            lt += 1;
            i += 1;
        } else if a[i] > v {
            a.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        } // now a[lo..lt-1] < v=a[lt..gt] < a[gt+1..hi].
    }
    _sort(a, lo, lt.saturating_sub(1));
    _sort(a, gt + 1, hi);
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
