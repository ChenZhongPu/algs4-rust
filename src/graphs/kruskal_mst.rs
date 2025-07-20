//! # Compute a minimum spanning forest using Krusal's algorithm and the union-find data type.
//!
//! The time complexity is O(E log(E)).

use crate::fundamentals::quick_union_uf::UF;

use super::{edge::Edge, weighted_graph::EdgeWeightedGraph};
pub struct KrusalMST {
    mst: Vec<Edge>, // a queue
    weight: f64,
}

impl KrusalMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut k_mst = KrusalMST {
            mst: vec![],
            weight: 0.0,
        };
        // create array of edges, sorted by weight
        // we can also use a min priority queue to sort implicitly.
        let mut edges = g.edges().collect::<Vec<Edge>>();
        edges.sort_unstable();
        let mut uf = UF::new(g.v());

        for edge in edges {
            if k_mst.mst.len() < g.v() - 1 {
                let v = edge.either();
                let w = edge.other(v);
                if !uf.connected(v, w) {
                    uf.union(v, w);
                    k_mst.weight += edge.weight();
                    k_mst.mst.push(edge);
                }
            } else {
                break;
            }
        }
        k_mst
    }

    /// Returns the sum of the edge weights in a minimum spanning tree (or forest).
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns the edges in a minimum spanning tree (or forest).
    pub fn edges(&self) -> std::vec::IntoIter<Edge> {
        self.mst.clone().into_iter()
    }
}

// TODO
//#[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn tiny_ewg() {
//         let mut g = EdgeWeightedGraph::new(8);
//         g.add_edge(Edge::new(4, 5, 0.35));
//         g.add_edge(Edge::new(4, 7, 0.37));
//         g.add_edge(Edge::new(5, 7, 0.28));
//         g.add_edge(Edge::new(0, 7, 0.16));
//         g.add_edge(Edge::new(1, 5, 0.32));
//         g.add_edge(Edge::new(0, 4, 0.38));
//         g.add_edge(Edge::new(2, 3, 0.17));
//         g.add_edge(Edge::new(1, 7, 0.19));
//         g.add_edge(Edge::new(0, 2, 0.26));
//         g.add_edge(Edge::new(1, 2, 0.36));
//         g.add_edge(Edge::new(1, 3, 0.29));
//         g.add_edge(Edge::new(2, 7, 0.34));
//         g.add_edge(Edge::new(6, 2, 0.40));
//         g.add_edge(Edge::new(3, 6, 0.52));
//         g.add_edge(Edge::new(6, 0, 0.58));
//         g.add_edge(Edge::new(6, 4, 0.93));
//
//         let mst = KrusalMST::new(&g);
//         mst.edges().for_each(|e| println!("{e}"));
//
//         assert_eq!(mst.weight(), 1.81);
//     }
// }
