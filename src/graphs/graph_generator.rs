//! # creating various graphs, including random bipartite graphs

use super::graph::Graph;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    v: usize,
    w: usize,
}

impl Edge {
    fn new(v: usize, w: usize) -> Edge {
        if v < w {
            Edge { v, w }
        } else {
            Edge { v: w, w: v }
        }
    }
}

/// Returns a random simple bipartite graph on `v1` and `v2` vertices with `e` edges.
pub fn bipartite(v1: usize, v2: usize, e: usize) -> Graph {
    assert!(e <= v1 * v2);
    let mut g = Graph::new(v1 + v2);
    let mut vertices: Vec<usize> = (0..v1 + v2).collect();
    vertices.shuffle(&mut thread_rng());

    let mut set = HashSet::new();
    let mut rng = thread_rng();
    let from = Uniform::from(0..v1);
    let to = Uniform::from(0..v2);
    while g.e() < e {
        let i = from.sample(&mut rng);
        let j = v1 + to.sample(&mut rng);
        let edge = Edge::new(i, j);
        if !set.contains(&edge) {
            set.insert(edge);
            g.add_edge(vertices[i], vertices[j]);
        }
    }
    g
}
