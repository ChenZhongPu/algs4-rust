//! # Quick Sort

use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(a: &mut [T]) {
    // optional: random shuffle `a` to eliminate dependence on input
    _sort(a, 0, a.len() - 1);
}

// note `j` can be `0`, so one solution is to let `lo` and `hi`  be `isize`.
// another solution is to check whether `j` is `0`.
//     if j > 0 {
//       _sort(a, lo, j - 1);
//      }
fn _sort<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let j = partition(a, lo, hi);

    _sort(a, lo, j.saturating_sub(1));
    _sort(a, j + 1, hi);
}

fn partition<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) -> usize {
    let (mut i, mut j) = (lo, hi + 1);

    loop {
        i += 1;
        while a[i] < a[lo] {
            if i == hi {
                break;
            } else {
                i += 1;
            }
        }
        j -= 1;
        while a[lo] < a[j] {
            if j == lo {
                break;
            } else {
                j -= 1;
            }
        }

        if i >= j {
            break;
        }

        a.swap(i, j);
    }

    a.swap(lo, j);
    j
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
