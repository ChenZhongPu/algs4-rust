//! # Heap sort
//!
//! copy to a `size + 1` array to enable 1-index based
use std::cmp::PartialOrd;

pub fn sort<T: Default + Copy + PartialOrd>(a: &mut [T]) {
    let mut arr = vec![T::default(); a.len() + 1];
    arr[1..].copy_from_slice(a); // 1-index based

    let mut n = a.len();

    // build a heap-sort array (max priority queue)
    for k in (1..=n / 2).rev() {
        sink(&mut arr, k);
    }

    // move the max to the end, and then sink
    while n > 1 {
        arr.swap(1, n);
        n -= 1;
        sink(&mut arr[..=n], 1);
    }

    // copy back
    a.copy_from_slice(&arr[1..]);
}

fn sink<T: Default + Copy + PartialOrd>(a: &mut [T], k: usize) {
    let mut index = k;
    let n = a.len() - 1;
    while 2 * index <= n {
        let mut j = 2 * index;
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
