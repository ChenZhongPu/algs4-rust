//! # Brute force three sum
//!
//! Counts the number of triples that sum to 0
//!
//! The time complexity is O(N^3)

pub fn count(a: &[i32]) -> usize {
    let n = a.len();
    let mut cnt = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 0 {
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
        let v = vec![0, -1, 2, -3, 1];
        // (0 -1 1), (2 -3 1)
        assert_eq!(count(&v), 2);
    }
}
