//! # Quick union of UF
//!
pub struct UF {
    id: Vec<usize>,
    count: usize,
}

impl UF {
    pub fn new(n: usize) -> UF {
        UF {
            id: (0..n).collect(),
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
        let p_root = self.find(p);
        let q_root = self.find(q);
        if p_root == q_root {
            return;
        }

        self.id[p_root] = q_root;
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
