//! # Dijkstra's algorithm. Computes the shortest path tree.
//! Assumes all weights are non-negative.

use crate::sorting::index_min_pq::IndexMinPQ;

use super::{edge::Edge, weighted_graph::EdgeWeightedGraph};

/// single-source shortest paths problem in edge-weighted graphs
/// where edge weights are non-negative.
/// This implementation uses Dijkstra's algorithm with a binary heap.
pub struct DijkstraUndirectedSP {
    dist_to: Vec<f64>,          // dist_to[v] = distance of shortest s -> v
    edge_to: Vec<Option<Edge>>, // edge_to[v] = last edge on shortest s -> v
    pq: IndexMinPQ<f64>,        // min priority queue of vertices
}

impl DijkstraUndirectedSP {
    pub fn new(g: &EdgeWeightedGraph, s: usize) -> Self {
        let mut sp = DijkstraUndirectedSP {
            dist_to: vec![f64::MAX; g.v()],
            edge_to: vec![None; g.v()],
            pq: IndexMinPQ::new(g.v()),
        };

        sp.dist_to[s] = 0.0;

        // relax vertices in order of distance from s
        sp.pq.insert(s, sp.dist_to[s]);
        while let Some(v) = sp.pq.del_min() {
            for edge in g.adj(v) {
                sp.relax(edge, v);
            }
        }

        sp
    }
    // relax edge e and update pq if changed
    fn relax(&mut self, e: Edge, v: usize) {
        let w = e.other(v);
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

    /// Returns the length of a shortest path between s and v
    pub fn dist_to(&self, v: usize) -> f64 {
        self.dist_to[v]
    }

    /// Returns true if there is a path between s and v
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] < f64::MAX
    }

    pub fn path_to(&self, v: usize) -> std::vec::IntoIter<Edge> {
        let mut path = Vec::new();
        if !self.has_path_to(v) {
            return path.into_iter();
        }

        let mut vertex = v;
        while let Some(edge) = self.edge_to[vertex].clone() {
            vertex = edge.other(vertex);
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

        let sp = DijkstraUndirectedSP::new(&g, 6);

        assert!((sp.dist_to(0) - 0.58).abs() < f64::EPSILON);
        sp.path_to(0).for_each(|e| print!("{};", e));
        println!();

        assert!((sp.dist_to(1) - 0.76).abs() < f64::EPSILON);
        sp.path_to(1).for_each(|e| print!("{};", e));
        println!();

        assert!((sp.dist_to(2) - 0.40).abs() < f64::EPSILON);
        assert!((sp.dist_to(5) - 1.02).abs() < f64::EPSILON);
        assert!((sp.dist_to(6) - 0.0).abs() < f64::EPSILON);
        assert!((sp.dist_to(7) - 0.74).abs() < f64::EPSILON);
    }
}
