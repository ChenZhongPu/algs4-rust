//!# Key-indexed counting uses `8N + 3R + 1` array accesses to stably sort `N` items whose keys are integers between 0 and R - 1.
//!

/// Key-indexed counting sort is an extremely effective and often overlooked sorting method for applications where keys are small integers.
pub struct KeyIndexedCount {
    data: Vec<usize>,
}

impl KeyIndexedCount {
    /// whose values are between 0 and r - 1
    pub fn new(data: Vec<usize>, r: usize) -> Self {
        let mut aux = vec![0usize; data.len()];
        let mut count = vec![0usize; r + 1];
        // compute frequency counts
        data.iter().for_each(|&key| count[key + 1] += 1);
        // transform counts to indices
        for _r in 0..r {
            count[_r + 1] += count[_r];
        }
        // distribute the records
        for i in 0..data.len() {
            aux[count[data[i]]] = data[i];
            count[data[i]] += 1;
        }
        KeyIndexedCount { data: aux }
    }

    /// after sorted data, just for ease of testing
    pub fn data(&self) -> Vec<usize> {
        self.data.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = vec![2, 3, 3, 4, 1, 3, 4, 3, 1, 2, 2, 1, 2, 4, 3, 4, 4, 2, 3, 4];
        let indexed_count = KeyIndexedCount::new(data, 5);
        assert_eq!(
            indexed_count.data(),
            vec![1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4]
        );
    }
}
