//! # Connected Component
//!
//! Compute connected components using depth first search.
use super::graph::Graph;

pub struct CC {
    marked: Vec<bool>,
    id: Vec<usize>,
    count: usize,
}

impl CC {
    pub fn new(g: &Graph) -> CC {
        let mut cc = CC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            count: 0,
        };
        for s in 0..g.v() {
            if !cc.marked[s] {
                cc.dfs(g, s);
                cc.count += 1;
            }
        }
        cc
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }

    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    pub fn id(&self, v: usize) -> usize {
        self.id[v]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cc() {
        let mut graph = Graph::new(13);
        graph.add_edge(0, 5);
        graph.add_edge(4, 3);
        graph.add_edge(0, 1);
        graph.add_edge(9, 12);
        graph.add_edge(6, 4);
        graph.add_edge(5, 4);
        graph.add_edge(0, 2);
        graph.add_edge(11, 12);
        graph.add_edge(9, 10);
        graph.add_edge(0, 6);
        graph.add_edge(7, 8);
        graph.add_edge(9, 11);
        graph.add_edge(5, 3);

        let cc = CC::new(&graph);

        assert_eq!(cc.count(), 3);

        let mut components = vec![Vec::new(); 3];
        for v in 0..graph.v() {
            components[cc.id(v)].push(v);
        }
        components.sort_unstable();
        components[0].sort_unstable();
        components[1].sort_unstable();
        components[2].sort_unstable();
        assert_eq!(
            components,
            vec![vec![0, 1, 2, 3, 4, 5, 6], vec![7, 8], vec![9, 10, 11, 12]]
        );
    }
}
