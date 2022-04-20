//!# Least significant digit first (LSD)

/// LSD string sort stably sorts fixed-length strings.
pub struct LSD;

impl LSD {
    /// To sort a list of strings that each have exactly `w` characters.
    /// We sort the strings `w` times with key-indexed counting, proceeding from the right to left.
    pub fn sort(a: &mut [&str], w: usize) {
        // sort `a` on leading `w` characters
        let n = a.len();
        let r = 256;
        let mut aux = vec![""; n];

        // sort by key-indexed counting on `dth` char
        for d in (0..w).rev() {
            let mut count = vec![0; r + 1];
            // compute frequency counts
            for i in 0..n {
                count[a[i].as_bytes()[d] as usize + 1] += 1;
            }

            // transform counts to indices
            for _r in 0..r {
                count[_r + 1] += count[_r];
            }

            // distribute
            for i in 0..n {
                aux[count[a[i].as_bytes()[d] as usize]] = a[i];
                count[a[i].as_bytes()[d] as usize] += 1;
            }

            // copy back
            a[..].clone_from_slice(&aux[..]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut data = vec![
            "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
            "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
        ];

        LSD::sort(&mut data, 7);

        assert_eq!(
            data,
            vec![
                "1ICK750", "1ICK750", "1OHV845", "1OHV845", "1OHV845", "2IYE230", "2RLA629",
                "2RLA629", "3ATW723", "3CIO720", "3CIO720", "4JZY524", "4PGC938"
            ]
        );
    }
}
