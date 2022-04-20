//!# MSD String sort
//!
//! Most-significant digit first string sort.
const R: usize = 256; // radix
const M: usize = 3; // cutoff for small sub-arrays
pub struct MSD;

impl MSD {
    fn char_at(s: &str, d: usize) -> i32 {
        if d < s.len() {
            s.as_bytes()[d] as i32
        } else {
            -1
        }
    }

    pub fn sort(a: &mut Vec<&str>) {
        let mut aux = vec![""; a.len()];
        MSD::_sort(a, &mut aux, 0, a.len() - 1, 0);
    }

    // sort from a[lo] to a[hi], starting at the d-th character
    fn _sort<'a>(a: &mut Vec<&'a str>, aux: &mut Vec<&'a str>, lo: usize, hi: usize, d: usize) {
        if hi <= lo + M {
            insert_sort(a, lo, hi, d);
            return;
        }
        let mut count = vec![0; R + 2];
        // computer frequency counts
        for i in lo..=hi {
            count[(MSD::char_at(a[i], d) + 2) as usize] += 1;
        }
        // transform counts to indices
        for r in 0..R + 1 {
            count[r + 1] += count[r];
        }
        // distribute
        for i in lo..=hi {
            aux[count[(MSD::char_at(a[i], d) + 1) as usize]] = a[i];
            count[(MSD::char_at(a[i], d) + 1) as usize] += 1;
        }
        // copy back
        a[lo..=hi].copy_from_slice(&aux[0..=hi - lo]);
        // recursively sort for each character value
        for r in 0..R {
            // `hi` may less than 0
            if (lo + count[r + 1]).saturating_sub(1) > lo + count[r] {
                MSD::_sort(a, aux, lo + count[r], lo + count[r + 1] - 1, d + 1);
            }
        }
    }
}

// sort from a[lo] to a[hi], starting at the dth character
fn insert_sort(a: &mut [&str], lo: usize, hi: usize, d: usize) {
    fn less(v: &str, w: &str, d: usize) -> bool {
        v[d..].cmp(&w[d..]).is_le()
    }

    for i in lo..=hi {
        let mut j = i;
        while j > lo && less(a[j], a[j - 1], d) {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut data = vec![
            "she",
            "sells",
            "seashells",
            "by",
            "the",
            "sea",
            "shore",
            "the",
            "shells",
            "she",
            "sells",
            "are",
            "surely",
            "seashells",
        ];
        MSD::sort(&mut data);

        assert_eq!(
            data,
            vec![
                "are",
                "by",
                "sea",
                "seashells",
                "seashells",
                "sells",
                "sells",
                "she",
                "she",
                "shells",
                "shore",
                "surely",
                "the",
                "the"
            ]
        );
    }
}
