//! # Insertion Sort
//!
//! It works well for partially sorted arrays, because the best case is (N - 1) compares and 0 exchanges.
use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(a: &mut [T]) {
    for i in 1..a.len() {
        // insert a[i] among a[i-1], a[i-2], a[i-3] ...
        let mut j = i;
        while j > 0 && a[j - 1] > a[j] {
            a.swap(j - 1, j);
            j -= 1;
        }
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
