//! # Determine single-source or multiple-source reachability in a digraph using depth first search.

use super::digraph::Digraph;

pub struct DirectedDFS {
    marked: Vec<bool>, // marked[v] = true iff v is reachable from source
    count: usize,      // number of vertices reachable from source
}

impl DirectedDFS {
    pub fn new(g: &Digraph, s: usize) -> DirectedDFS {
        let mut directed_dfs = DirectedDFS {
            marked: vec![false; g.v()],
            count: 0,
        };
        directed_dfs.dfs(g, s);
        directed_dfs
    }

    pub fn from_sources(g: &Digraph, sources: Vec<usize>) -> DirectedDFS {
        let mut directed_dfs = DirectedDFS {
            marked: vec![false; g.v()],
            count: 0,
        };
        for v in sources {
            if !directed_dfs.marked(v) {
                directed_dfs.dfs(g, v);
            }
        }
        directed_dfs
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.count += 1;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }

    /// Is there a directed path from the source vertex to v?
    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    /// Returns the number of vertices reachable from the source vertex
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_dg() {
        let mut digraph = Digraph::new(13);
        digraph.add_edge(4, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 2);
        digraph.add_edge(6, 0);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 0);
        digraph.add_edge(11, 12);
        digraph.add_edge(12, 9);
        digraph.add_edge(9, 10);
        digraph.add_edge(9, 11);
        digraph.add_edge(8, 9);
        digraph.add_edge(10, 12);
        digraph.add_edge(11, 4);
        digraph.add_edge(4, 3);
        digraph.add_edge(3, 5);
        digraph.add_edge(7, 8);
        digraph.add_edge(8, 7);
        digraph.add_edge(5, 4);
        digraph.add_edge(0, 5);
        digraph.add_edge(6, 4);
        digraph.add_edge(6, 9);
        digraph.add_edge(7, 6);

        let dfs = DirectedDFS::new(&digraph, 1);
        assert_eq!(dfs.count(), 1);
        let mut tmp = Vec::new();
        for v in 0..digraph.v() {
            if dfs.marked(v) {
                tmp.push(v);
            }
        }
        assert_eq!(tmp, vec![1]);

        let dfs = DirectedDFS::new(&digraph, 2);
        assert_eq!(dfs.count(), 6);
        let mut tmp = Vec::new();
        for v in 0..digraph.v() {
            if dfs.marked(v) {
                tmp.push(v);
            }
        }
        tmp.sort_unstable();
        assert_eq!(tmp, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn multi_sources() {
        let mut digraph = Digraph::new(13);
        digraph.add_edge(4, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 2);
        digraph.add_edge(6, 0);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 0);
        digraph.add_edge(11, 12);
        digraph.add_edge(12, 9);
        digraph.add_edge(9, 10);
        digraph.add_edge(9, 11);
        digraph.add_edge(8, 9);
        digraph.add_edge(10, 12);
        digraph.add_edge(11, 4);
        digraph.add_edge(4, 3);
        digraph.add_edge(3, 5);
        digraph.add_edge(7, 8);
        digraph.add_edge(8, 7);
        digraph.add_edge(5, 4);
        digraph.add_edge(0, 5);
        digraph.add_edge(6, 4);
        digraph.add_edge(6, 9);
        digraph.add_edge(7, 6);

        let dfs = DirectedDFS::from_sources(&digraph, vec![1, 2, 6]);
        assert_eq!(dfs.count(), 11);
        let mut tmp = (0..digraph.v())
            .filter(|&x| dfs.marked(x))
            .collect::<Vec<usize>>();
        tmp.sort_unstable();
        assert_eq!(tmp, vec![0, 1, 2, 3, 4, 5, 6, 9, 10, 11, 12]);
    }
}
