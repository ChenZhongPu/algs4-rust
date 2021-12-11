//! # Selection Sort
//!
//! It works by repeatedly selecting the smallest remaining item.
use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        let mut min = i;
        for j in (i + 1)..n {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(i, min);
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
