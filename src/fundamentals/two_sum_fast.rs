//! # Two sum fast
//!
//! Using binary search
//!
//! The time complexity is O(N logN)

use crate::fundamentals::binary_search;

pub fn count(a: &mut [i32]) -> usize {
    a.sort_unstable();
    let n = a.len();
    let mut cnt = 0;
    for i in 0..n {
        if let Some(j) = binary_search::index_of(-a[i], a) {
            if j > i {
                cnt += 1;
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum() {
        let mut v = vec![7, 1, 2, -7, 4, 0, -3, 3];
        assert_eq!(count(&mut v), 2);
    }
}
