//! # Compute a minimum spanning forest using a lazy version of Prim's algorithm.
//!
//! Note that if all weights are distinct, the MST is unique.
//! The time complexity is O(E log(E)).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::{edge::Edge, weighted_graph::EdgeWeightedGraph};
pub struct LazyPrimMST {
    weight: f64,                   // total weight of MST
    mst: Vec<Edge>, // edges in MST: a queue, but since only `enqueue` is used, we can use `Vec`.
    marked: Vec<bool>, // marked[v] = true iff v on tree
    pq: BinaryHeap<Reverse<Edge>>, // a min priority heap
}

impl LazyPrimMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut prim_mst = LazyPrimMST {
            weight: 0.0,
            mst: vec![],
            marked: vec![false; g.v()],
            pq: BinaryHeap::new(),
        };
        for v in 0..g.v() {
            if !prim_mst.marked[v] {
                prim_mst.prim(g, v);
            }
        }
        prim_mst
    }

    fn prim(&mut self, g: &EdgeWeightedGraph, s: usize) {
        self.scan(g, s);
        while let Some(Reverse(e)) = self.pq.pop() {
            let v = e.either();
            let w = e.other(v);
            assert!(self.marked[v] || self.marked[w]);
            if self.marked[v] == self.marked[w] {
                // lazy, both v and w already scanned
                continue;
            }
            self.mst.push(e.clone());
            self.weight += e.weight();
            if !self.marked[v] {
                // v becomes part of tree
                self.scan(g, v);
            }
            if !self.marked[w] {
                // w becomes part of tree
                self.scan(g, w);
            }
        }
    }

    // add all edges e incident to v onto pq
    // if the other end point has not yet been scanned
    fn scan(&mut self, g: &EdgeWeightedGraph, v: usize) {
        assert!(!self.marked[v]);
        self.marked[v] = true;
        for edge in g.adj(v) {
            if !self.marked[edge.other(v)] {
                self.pq.push(Reverse(edge));
            }
        }
    }

    /// Returns the sum of the edge weights in a minimum spanning tree
    /// (or forest)
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns the edges in a minimum spanning tree (or forest).
    pub fn edges(&self) -> std::vec::IntoIter<Edge> {
        self.mst.clone().into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_ewg() {
        let mut g = EdgeWeightedGraph::new(8);
        g.add_edge(Edge::new(4, 5, 0.35));
        g.add_edge(Edge::new(4, 7, 0.37));
        g.add_edge(Edge::new(5, 7, 0.28));
        g.add_edge(Edge::new(0, 7, 0.16));
        g.add_edge(Edge::new(1, 5, 0.32));
        g.add_edge(Edge::new(0, 4, 0.38));
        g.add_edge(Edge::new(2, 3, 0.17));
        g.add_edge(Edge::new(1, 7, 0.19));
        g.add_edge(Edge::new(0, 2, 0.26));
        g.add_edge(Edge::new(1, 2, 0.36));
        g.add_edge(Edge::new(1, 3, 0.29));
        g.add_edge(Edge::new(2, 7, 0.34));
        g.add_edge(Edge::new(6, 2, 0.40));
        g.add_edge(Edge::new(3, 6, 0.52));
        g.add_edge(Edge::new(6, 0, 0.58));
        g.add_edge(Edge::new(6, 4, 0.93));

        let mst = LazyPrimMST::new(&g);
        mst.edges().for_each(|e| println!("{}", e));

        assert_eq!(mst.weight(), 1.81);
    }
}
