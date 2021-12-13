//! # Heap sort
//!
//! In-place sorting, 0-index based
use std::cmp::PartialOrd;

pub fn sort<T: Default + Copy + PartialOrd>(a: &mut [T]) {
    let n = a.len();
    // heapify phase
    for k in (0..=(n / 2) - 1).rev() {
        sink(a, k);
    }
    // sortdown phase
    let mut k = n - 1;
    while k > 0 {
        a.swap(0, k);
        k -= 1;
        sink(&mut a[..=k], 0);
    }
}

fn sink<T: PartialOrd>(a: &mut [T], k: usize) {
    let mut index = k;
    let n = a.len() - 1;
    while 2 * index < n {
        // 2 * index + 1 <= n
        let mut j = 2 * index + 1;
        if j < n && a[j] < a[j + 1] {
            j += 1;
        }
        if a[index] > a[j] {
            break;
        }
        a.swap(index, j);
        index = j;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut v = vec!['S', 'O', 'R', 'T', 'E', 'X', 'A', 'M', 'P', 'L', 'E'];
        sort(&mut v);
        assert_eq!(
            v,
            vec!['A', 'E', 'E', 'L', 'M', 'O', 'P', 'R', 'S', 'T', 'X']
        );
    }
}
