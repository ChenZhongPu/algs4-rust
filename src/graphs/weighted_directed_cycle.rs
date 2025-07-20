use super::{directed_edge::DirectedEdge, weighted_digraph::EdgeWeightedDiagraph};

/// Determining whether an edge-weighted digraph has a directed cycle.
/// This implementation uses depth first search.
pub struct EdgeWeightedDirectedCycle {
    marked: Vec<bool>,          //  has vertex v been visited?
    on_stack: Vec<bool>,        // is vertex on the stack?
    edge_to: Vec<DirectedEdge>, // previous edge on path to v
    cycle: Vec<DirectedEdge>,   // directed cycle
}

impl EdgeWeightedDirectedCycle {
    pub fn new(g: &EdgeWeightedDiagraph) -> Self {
        let mut dc = EdgeWeightedDirectedCycle {
            marked: vec![false; g.v()],
            on_stack: vec![false; g.v()],
            edge_to: vec![DirectedEdge::default(); g.v()],
            cycle: vec![],
        };
        for v in 0..g.v() {
            if !dc.marked[v] && dc.cycle.is_empty() {
                dc.dfs(g, v);
            }
        }
        dc
    }

    fn dfs(&mut self, g: &EdgeWeightedDiagraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;

        for e in g.adj(v) {
            let w = e.to();

            // short circuit if directed cycle found
            if !self.cycle.is_empty() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = e;
                self.dfs(g, w);
            } else if self.on_stack[w] {
                // trace back directed cycle
                let mut f = e;
                while f.from() != w {
                    self.cycle.push(f);
                    f = self.edge_to[f.from()];
                }
                self.cycle.push(f);
            }
        }
        self.on_stack[v] = false;
    }

    /// Does the digraph have a directed cycle?
    pub fn has_cycle(&self) -> bool {
        !self.cycle.is_empty()
    }

    pub fn cycle(&self) -> std::vec::IntoIter<DirectedEdge> {
        let mut paths = self.cycle.clone();
        paths.reverse();
        paths.into_iter()
    }
}

// TODO
//#[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn tiny_ewg() {
//         let mut g = EdgeWeightedDiagraph::new(8);
//         g.add_edge(DirectedEdge::new(4, 5, 0.35));
//         g.add_edge(DirectedEdge::new(5, 4, 0.35));
//         g.add_edge(DirectedEdge::new(4, 7, 0.37));
//         g.add_edge(DirectedEdge::new(5, 7, 0.28));
//         g.add_edge(DirectedEdge::new(7, 5, 0.28));
//         g.add_edge(DirectedEdge::new(5, 1, 0.32));
//         g.add_edge(DirectedEdge::new(0, 4, 0.38));
//         g.add_edge(DirectedEdge::new(0, 2, 0.26));
//         g.add_edge(DirectedEdge::new(7, 3, 0.39));
//         g.add_edge(DirectedEdge::new(1, 3, 0.29));
//         g.add_edge(DirectedEdge::new(2, 7, 0.34));
//         g.add_edge(DirectedEdge::new(6, 2, 0.40));
//         g.add_edge(DirectedEdge::new(3, 6, 0.52));
//         g.add_edge(DirectedEdge::new(6, 0, 0.58));
//         g.add_edge(DirectedEdge::new(6, 4, 0.93));
//
//         let dc = EdgeWeightedDirectedCycle::new(&g);
//
//         assert!(dc.has_cycle());
//     }
//
//     #[test]
//     fn tiny_ewdag() {
//         let mut g = EdgeWeightedDiagraph::new(8);
//         g.add_edge(DirectedEdge::new(5, 4, 0.35));
//         g.add_edge(DirectedEdge::new(4, 7, 0.37));
//         g.add_edge(DirectedEdge::new(5, 7, 0.28));
//         g.add_edge(DirectedEdge::new(5, 1, 0.32));
//         g.add_edge(DirectedEdge::new(4, 0, 0.38));
//         g.add_edge(DirectedEdge::new(0, 2, 0.26));
//         g.add_edge(DirectedEdge::new(3, 7, 0.39));
//         g.add_edge(DirectedEdge::new(1, 3, 0.29));
//         g.add_edge(DirectedEdge::new(7, 2, 0.34));
//         g.add_edge(DirectedEdge::new(6, 2, 0.40));
//         g.add_edge(DirectedEdge::new(3, 6, 0.52));
//         g.add_edge(DirectedEdge::new(6, 0, 0.58));
//         g.add_edge(DirectedEdge::new(6, 4, 0.93));
//
//         let dc = EdgeWeightedDirectedCycle::new(&g);
//
//         assert!(!dc.has_cycle());
//     }
// }
