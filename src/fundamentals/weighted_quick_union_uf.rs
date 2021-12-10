//! # Weighted quick-union
//!
//! The worst-case order of growth of cost is logN.
pub struct UF {
    id: Vec<usize>, // parent link
    sz: Vec<usize>, // size of component for roots
    count: usize,
}

impl UF {
    pub fn new(n: usize) -> UF {
        UF {
            id: (0..n).collect(),
            sz: vec![1; n],
            count: n,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn find(&self, p: usize) -> usize {
        let mut component = p;
        while component != self.id[component] {
            component = self.id[component];
        }
        component
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i == j {
            return;
        }

        // make smaller root point to larger one
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_uf() {
        let mut uf = UF::new(10);
        let data = vec![
            (4, 3),
            (3, 8),
            (6, 5),
            (9, 4),
            (2, 1),
            (8, 9),
            (5, 0),
            (7, 2),
            (6, 1),
            (1, 0),
            (6, 7),
        ];
        for (i, j) in data {
            if uf.connected(i, j) {
                continue;
            }
            uf.union(i, j);
        }

        assert_eq!(uf.count(), 2);
    }
}
