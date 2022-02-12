//! # Determining a topological order of a directed acyclic graph (DGA).
//!
//! A digraph has a topological order if and only if it is a DAG.
//! This implementation is queue-based in a non-recursive way.
//!
//! Note the result relies on the order of adj.

use super::digraph::Digraph;
use std::collections::VecDeque;

pub struct TopologicalX {
    order: Vec<usize>, // vertices in topological order
    ranks: Vec<usize>, // ranks[v] = order where v appears in order
}

impl TopologicalX {
    pub fn new(g: &Digraph) -> Self {
        let mut in_degree: Vec<i32> = (0..g.v()).map(|v| g.in_degree(v) as i32).collect();

        let mut order = Vec::new();
        let mut ranks = vec![0; g.v()];
        let mut count = 0;

        // initialize queue to contain all vertices with indegree = 0
        let mut queue: VecDeque<usize> = (0..g.v()).filter(|v| in_degree[*v] == 0).collect();

        while let Some(v) = queue.pop_front() {
            order.push(v);
            ranks[v] = count;
            count += 1;
            for w in g.adj(v).clone() {
                in_degree[w] -= 1;
                if in_degree[w] == 0 {
                    queue.push_back(w);
                }
            }
        }
        // there is a directed cycle
        if count != g.v() {
            order.clear();
            ranks.clear();
        }

        TopologicalX { order, ranks }
    }

    pub fn has_order(&self) -> bool {
        !self.order.is_empty()
    }

    pub fn order(&self) -> std::vec::IntoIter<usize> {
        self.order.clone().into_iter()
    }

    pub fn rank(&self, v: usize) -> Option<usize> {
        if self.has_order() {
            Some(self.ranks[v])
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

        let topological = TopologicalX::new(&dag);
        assert_eq!(topological.has_order(), true);
        assert_eq!(
            topological.order().collect::<Vec<usize>>(),
            vec![2, 8, 3, 0, 7, 1, 5, 6, 4, 9, 11, 10, 12]
        );
    }

    #[test]
    fn not_dag() {
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

        let topological = TopologicalX::new(&digraph);
        assert_eq!(topological.has_order(), false);
    }
}
