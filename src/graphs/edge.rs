//! # A weighted edge in an `EdgeWeightedGraph`.
//!
//! Each edge consists of two integers (naming the two vertices)
//! and a real-value weight.
//!
//! Compares two edges by weight.
#[derive(Debug, Clone)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: f64,
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Edge { v, w, weight }
    }

    /// Returns the weight of this edge.
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns either endpoint of this edge.
    pub fn either(&self) -> usize {
        self.v
    }

    /// Returns the endpoint of this edge that is different from the given vertex.
    pub fn other(&self, v: usize) -> usize {
        if v == self.v {
            self.w
        } else if v == self.w {
            self.v
        } else {
            panic!("Illegal endpoint");
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Edge {}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} {:.5}", self.v, self.w, self.weight)
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn one_edge() {
        let edge = Edge::new(12, 34, 5.67);

        let edge2 = Edge::new(10, 11, 5.67);
        assert_eq!(edge.partial_cmp(&edge2), Some(Ordering::Equal));
        assert_eq!(edge, edge2);

        let edge3 = Edge::new(10, 11, 8.0);
        assert_eq!(edge.partial_cmp(&edge3), Some(Ordering::Less));
        assert!(edge < edge3);
    }
}
