//! # Three sum fast
//!
//! Using binary search
//!
//! The time complexity is O(N^2 logN)

use crate::fundamentals::binary_search;

pub fn count(a: &mut [i32]) -> usize {
    a.sort_unstable();
    let n = a.len();
    let mut cnt = 0;
    for i in 0..n {
        for j in i + 1..n {
            if let Some(k) = binary_search::index_of(-a[i] - a[j], a) {
                if k > j {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum() {
        let mut v = vec![0, -1, 2, -3, 1];
        // (0 -1 1), (2 -3 1)
        assert_eq!(count(&mut v), 2);
    }
}
