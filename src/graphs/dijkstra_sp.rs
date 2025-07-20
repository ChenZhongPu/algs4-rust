//! # The single source shortest paths problem in edge-weighted digraphs
//! where the edge weights are non-negative.
//!
//! This implementation uses Dijkstra's algorithm with a binary heap.
//! The time complexity is O(E log(V))

use std::vec;

use crate::sorting::index_min_pq::IndexMinPQ;

use super::{directed_edge::DirectedEdge, weighted_digraph::EdgeWeightedDiagraph};
pub struct DijkstraSP {
    dist_to: Vec<f64>,                  // dist_to[v] = distance of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on shortest s->v path
    pq: IndexMinPQ<f64>,                // min priority queue of vertices
}

impl DijkstraSP {
    pub fn new(g: &EdgeWeightedDiagraph, s: usize) -> Self {
        let mut sp = DijkstraSP {
            dist_to: vec![f64::MAX; g.v()],
            edge_to: vec![None; g.v()],
            pq: IndexMinPQ::new(g.v()),
        };

        sp.dist_to[s] = 0.0;

        // relax vertices in order of distance from s
        sp.pq.insert(s, sp.dist_to[s]);
        while let Some(v) = sp.pq.del_min() {
            for edge in g.adj(v) {
                sp.relax(edge);
            }
        }
        sp
    }

    fn relax(&mut self, e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if self.dist_to[w] > self.dist_to[v] + e.weight() {
            self.dist_to[w] = self.dist_to[v] + e.weight();
            self.edge_to[w] = Some(e);
            if self.pq.contains(w) {
                self.pq.decrease_key(w, self.dist_to[w]);
            } else {
                self.pq.insert(w, self.dist_to[w]);
            }
        }
    }

    /// Returns the length of a shortest path from s to v
    pub fn dist_to(&self, v: usize) -> f64 {
        self.dist_to[v]
    }

    /// Returns true if there is a path from s to v
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] < f64::MAX
    }

    // we can also trace back the path by checking the source
    pub fn path_to(&self, v: usize) -> std::vec::IntoIter<DirectedEdge> {
        let mut path = Vec::new();
        if !self.has_path_to(v) {
            return path.into_iter();
        }

        let mut vertex = v;
        while let Some(edge) = self.edge_to[vertex] {
            vertex = edge.from();
            path.push(edge);
        }

        path.reverse();
        path.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_ewg() {
        let mut g = EdgeWeightedDiagraph::new(8);
        g.add_edge(DirectedEdge::new(4, 5, 0.35));
        g.add_edge(DirectedEdge::new(5, 4, 0.35));
        g.add_edge(DirectedEdge::new(4, 7, 0.37));
        g.add_edge(DirectedEdge::new(5, 7, 0.28));
        g.add_edge(DirectedEdge::new(7, 5, 0.28));
        g.add_edge(DirectedEdge::new(5, 1, 0.32));
        g.add_edge(DirectedEdge::new(0, 4, 0.38));
        g.add_edge(DirectedEdge::new(0, 2, 0.26));
        g.add_edge(DirectedEdge::new(7, 3, 0.39));
        g.add_edge(DirectedEdge::new(1, 3, 0.29));
        g.add_edge(DirectedEdge::new(2, 7, 0.34));
        g.add_edge(DirectedEdge::new(6, 2, 0.40));
        g.add_edge(DirectedEdge::new(3, 6, 0.52));
        g.add_edge(DirectedEdge::new(6, 0, 0.58));
        g.add_edge(DirectedEdge::new(6, 4, 0.93));

        let sp = DijkstraSP::new(&g, 0);

        assert!((sp.dist_to(0) - 0.0).abs() < f64::EPSILON);

        assert!((sp.dist_to(1) - 1.05).abs() < f64::EPSILON);
        sp.path_to(1).for_each(|x| print!("{x};"));
        println!();

        assert!((sp.dist_to(2) - 0.26).abs() < f64::EPSILON);
        assert!((sp.dist_to(3) - 0.99).abs() < f64::EPSILON);

        assert!((sp.dist_to(4) - 0.38).abs() < f64::EPSILON);
        assert!((sp.dist_to(5) - 0.73).abs() < f64::EPSILON);
    }
}
