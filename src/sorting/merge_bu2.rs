//! # Bottom-up Merge Sort

use std::cmp::PartialOrd;

fn merge<T: Copy + PartialOrd>(a: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    let (mut i, mut j) = (lo, mid + 1);

    aux[lo..(hi + 1)].copy_from_slice(&a[lo..(hi + 1)]);

    for x in a.iter_mut().take(hi + 1).skip(lo) {
        if i > mid {
            *x = aux[j];
            j += 1;
        } else if j > hi {
            *x = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            *x = aux[j];
            j += 1;
        } else {
            *x = aux[i];
            i += 1;
        }
    }
}

pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    let n = a.len();
    let mut aux = a.to_vec();

    let mut sz = 1;
    while sz < n {
        let mut lo = 0;
        while lo < n - sz {
            merge(
                a,
                &mut aux,
                lo,
                lo + sz - 1,
                std::cmp::min(lo + sz + sz - 1, n - 1),
            );
            lo += sz + sz;
        }
        sz = sz + sz;
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
