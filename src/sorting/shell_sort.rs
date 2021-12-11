//! # Shell Sort
//!
//! Shellsort is a simple extension of insertion sort that gains speed by allowing exchanges of array entries that are far apart, to produce partially sorted arrays that can be efficiently sorted, eventually by insertion sort.
use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    let mut h = 1;

    while h < n / 3 {
        h = 3 * h + 1; // 1, 4, 13, 40, 121, 364, 1093, ...
    }
    while h >= 1 {
        // h-sort the array
        for i in h..n {
            let mut j = i;
            while j >= h && a[j] < a[j - h] {
                a.swap(j, j - h);
                j -= h;
            }
        }
        h /= 3;
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
