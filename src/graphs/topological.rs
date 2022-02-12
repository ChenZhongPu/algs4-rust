//! # Determining a topological order of a directed acyclic graph (DGA).
//!
//! A digraph has a topological order if and only if it is a DAG.
//! This implementation uses depth-first search.

use super::{dfs_order::DepthFirstOrder, digraph::Digraph, directed_cycle::DirectedCycle};
pub struct Topological {
    order: Vec<usize>, // topological order
    rank: Vec<usize>,  // rank[v] = rank of v in order
}

impl Topological {
    pub fn new(g: &Digraph) -> Self {
        let finder = DirectedCycle::new(g);
        let mut order = vec![];
        let mut rank = vec![];
        if !finder.has_cycle() {
            let dfs = DepthFirstOrder::new(g);
            order = dfs.rev_post().collect::<Vec<usize>>();
            rank = vec![0; g.v()];
            for (i, v) in order.iter().enumerate() {
                rank[*v] = i;
            }
        }
        Topological { order, rank }
    }

    /// Does the digraph have a topological order?
    pub fn has_order(&self) -> bool {
        !self.order.is_empty()
    }

    /// Returns a topological order if the digraph has a topologial order
    pub fn order(&self) -> std::vec::IntoIter<usize> {
        self.order.clone().into_iter()
    }

    /// The the rank of vertex
    pub fn rank(&self, v: usize) -> Option<usize> {
        if self.has_order() {
            Some(self.rank[v])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dag() {
        let mut dag = Digraph::new(13);
        dag.add_edge(2, 3);
        dag.add_edge(0, 6);
        dag.add_edge(0, 1);
        dag.add_edge(2, 0);
        dag.add_edge(11, 12);
        dag.add_edge(9, 12);
        dag.add_edge(9, 11);
        dag.add_edge(9, 10);
        dag.add_edge(3, 5);
        dag.add_edge(8, 7);
        dag.add_edge(5, 4);
        dag.add_edge(0, 5);
        dag.add_edge(6, 4);
        dag.add_edge(6, 9);
        dag.add_edge(7, 6);

        let topological = Topological::new(&dag);
        assert_eq!(topological.has_order(), true);
        assert_eq!(
            topological.order().collect::<Vec<usize>>(),
            vec![8, 7, 2, 3, 0, 5, 1, 6, 9, 10, 11, 12, 4]
        );
    }
}
