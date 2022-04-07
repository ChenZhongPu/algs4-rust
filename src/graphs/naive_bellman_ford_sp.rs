//! # Bellman-Ford shortest path algorithm.
//! Single-source shortest path from a give source `s` for any edge
//! weighted digraph and no negative cycles reachable from `s`.
//!  It takes time proportional to EV.

use super::{directed_edge::DirectedEdge, weighted_digraph::EdgeWeightedDiagraph};
/// Initialized `dist_to[s]` to 0 and all other values to infinity.
/// Then, considering the digraph's edges in any order, relax all edges. Make `V` such passes.
pub struct NaiveBellmanFordSP {
    dist_to: Vec<f64>,
    s: usize,
}

impl NaiveBellmanFordSP {
    pub fn new(g: &EdgeWeightedDiagraph, s: usize) -> Self {
        let mut sp = NaiveBellmanFordSP {
            dist_to: vec![f64::MAX; g.v()],
            s,
        };
        sp.dist_to[s] = 0.0;
        for _ in 0..g.v() {
            // `V` pass
            for v in 0..g.v() {
                for e in g.adj(v) {
                    sp.relax(&e);
                }
            }
        }
        sp
    }

    fn relax(&mut self, e: &DirectedEdge) {
        let from = e.from();
        let to = e.to();
        if self.dist_to[from] + e.weight() < self.dist_to[to] {
            self.dist_to[to] = self.dist_to[from] + e.weight();
        }
    }

    pub fn dist_to(&self, v: usize) -> f64 {
        self.dist_to[v]
    }

    pub fn source(&self) -> usize {
        self.s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sp() {
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

        let sp = NaiveBellmanFordSP::new(&g, 0);

        assert_eq!(sp.dist_to(0), 0.0);
        assert_eq!(sp.dist_to(1), 1.05);
        assert_eq!(sp.dist_to(2), 0.26);
        assert!((sp.dist_to(3) - 0.99).abs() < f64::EPSILON);

        assert!((sp.dist_to(4) - 0.38).abs() < f64::EPSILON);
        assert!((sp.dist_to(5) - 0.73).abs() < f64::EPSILON);
    }
}
