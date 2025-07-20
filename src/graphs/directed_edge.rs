//!# A weight edge in an EdgeWeightedDiagraph.
//!
//! Each edge consists of two integers (naming the two vertices) and a
//! real-value weight.
#[derive(Clone, Debug, Copy, Default)]
pub struct DirectedEdge {
    v: usize,
    w: usize,
    weight: f64,
}

impl DirectedEdge {
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        DirectedEdge { v, w, weight }
    }

    /// Returns the tail vertex of the directed edge.
    pub fn from(&self) -> usize {
        self.v
    }

    /// Returns the head vertex of the directed edge.
    pub fn to(&self) -> usize {
        self.w
    }

    /// Returns a string representation of the directed edge.
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl std::fmt::Display for DirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{} {:5.2}", self.v, self.w, self.weight)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edge() {
        let edge = DirectedEdge::new(12, 34, 5.67);
        println!("{edge}");
    }
}
