//! # Merge Sort
//!
//! Divide-and-conquer: sort the left half and right half, and then merge
use std::cmp::PartialOrd;

pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    let mut aux = a.to_vec();
    _sort(a, &mut aux, 0, a.len() - 1);
}

fn merge<T: Copy + PartialOrd>(a: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    let (mut i, mut j) = (lo, mid + 1);

    aux[lo..(hi + 1)].copy_from_slice(&a[lo..(hi + 1)]);

    // `cargo clippy` would complain:
    // for k in lo..=hi {
    //     // merge back to a[lo..hi]
    //     if i > mid {
    //         a[k] = aux[j];
    //         j += 1;
    //     } else if j > hi {
    //         a[k] = aux[i];
    //         i += 1;
    //     } else if aux[j] < aux[i] {
    //         a[k] = aux[j];
    //         j += 1;
    //     } else {
    //         a[k] = aux[i];
    //         i += 1;
    //     }
    // }
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

fn _sort<T: Copy + PartialOrd>(a: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    _sort(a, aux, lo, mid);
    _sort(a, aux, mid + 1, hi);
    merge(a, aux, lo, mid, hi);
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
