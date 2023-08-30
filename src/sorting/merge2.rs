//! # Merge Sort 2
//!
//! Divide-and-conquer: sort the left half and right half, and then merge
// use std::cmp::PartialOrd;

pub fn sort<T: Copy + PartialOrd>(a: &mut [T]) {
    let mut aur = a.to_vec(); // allocate space only once (init values are not important)
    merge_sort(a, 0, a.len() - 1, &mut aur);
}

fn merge<T: Copy + PartialOrd>(a: &mut [T], lo: usize, mid: usize, hi: usize, aux: &mut [T]) {
    aux[lo..=hi].copy_from_slice(&a[lo..=hi]);

    let mut i = lo;
    let mut j = mid + 1;

    for v in a.iter_mut().take(hi + 1).skip(lo) {
        if i > mid {
            *v = aux[j];
            j += 1;
        } else if j > hi {
            *v = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            *v = aux[j];
            j += 1;
        } else {
            *v = aux[i];
            i += 1;
        }
    }
}

fn merge_sort<T: Copy + PartialOrd>(a: &mut [T], lo: usize, hi: usize, aur: &mut [T]) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort(a, lo, mid, aur);
    merge_sort(a, mid + 1, hi, aur);
    merge(a, lo, mid, hi, aur);
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
