//! # Computes shortest paths in an edge weighted acyclic digraph.

use super::{
    directed_edge::DirectedEdge, topological::Topological, weighted_digraph::EdgeWeightedDiagraph,
};

/// Solving the single-source shortest paths problem in edge-weighted acyclic
/// graphs (DAGs). The edge weights can be positive, negative, or zero.
///
/// This implementation uses a topological-sort based algorithm.
/// The time complexity is O(V + E).
pub struct AcyclicSP {
    dist_to: Vec<f64>,          // dist_to[v] = distance of shortest s->v path
    edge_to: Vec<DirectedEdge>, // edge_to[v] = last edge on shortest s->v path
    s: usize,
}

impl AcyclicSP {
    pub fn new(g: &EdgeWeightedDiagraph, s: usize) -> Self {
        let mut sp = AcyclicSP {
            dist_to: vec![f64::MAX; g.v()],
            edge_to: vec![DirectedEdge::default(); g.v()],
            s,
        };
        sp.dist_to[s] = 0.0;

        // visit vertices in topological order
        let topological = Topological::from_weighted_diagraph(g);
        if !topological.has_order() {
            panic!("Digraph is not acyclic");
        }
        for v in topological.order() {
            for e in g.adj(v) {
                sp.relax(e);
            }
        }
        sp
    }

    fn relax(&mut self, e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if self.dist_to[w] > self.dist_to[v] + e.weight() {
            self.dist_to[w] = self.dist_to[v] + e.weight();
            self.edge_to[w] = e;
        }
    }

    pub fn dist_to(&self, v: usize) -> f64 {
        self.dist_to[v]
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] < f64::MAX
    }

    pub fn path_to(&self, v: usize) -> std::vec::IntoIter<DirectedEdge> {
        let mut path = Vec::new();
        if !self.has_path_to(v) {
            return path.into_iter();
        }
        let mut p = v;
        while p != self.s {
            let e = self.edge_to[p];
            p = e.from();
            path.push(e);
        }
        path.reverse();
        path.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_ewdag() {
        let mut g = EdgeWeightedDiagraph::new(8);
        g.add_edge(DirectedEdge::new(5, 4, 0.35));
        g.add_edge(DirectedEdge::new(4, 7, 0.37));
        g.add_edge(DirectedEdge::new(5, 7, 0.28));
        g.add_edge(DirectedEdge::new(5, 1, 0.32));
        g.add_edge(DirectedEdge::new(4, 0, 0.38));
        g.add_edge(DirectedEdge::new(0, 2, 0.26));
        g.add_edge(DirectedEdge::new(3, 7, 0.39));
        g.add_edge(DirectedEdge::new(1, 3, 0.29));
        g.add_edge(DirectedEdge::new(7, 2, 0.34));
        g.add_edge(DirectedEdge::new(6, 2, 0.40));
        g.add_edge(DirectedEdge::new(3, 6, 0.52));
        g.add_edge(DirectedEdge::new(6, 0, 0.58));
        g.add_edge(DirectedEdge::new(6, 4, 0.93));

        let sp = AcyclicSP::new(&g, 5);
        assert!(sp.dist_to(0) - 0.73 < f64::EPSILON);
        assert!(sp.dist_to(1) - 0.32 < f64::EPSILON);
        assert!(sp.dist_to(2) - 0.62 < f64::EPSILON);
        assert!(sp.dist_to(3) - 0.61 < f64::EPSILON);
        assert!(sp.dist_to(4) - 0.35 < f64::EPSILON);
    }
}
