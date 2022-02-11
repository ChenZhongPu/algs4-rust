//! # A directed graph of vertices named 0 through v-1.
//!
//! This implementation uses an `adjacency-lists` representation.
use std::fmt;
pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>,
    in_degree: Vec<usize>,
}

impl Digraph {
    /// Initializes an empty digraph with v vertices.
    pub fn new(v: usize) -> Digraph {
        Digraph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
            in_degree: vec![0; v],
        }
    }

    /// Returns the number of vertices in this digraph.
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in this digraph.
    pub fn e(&self) -> usize {
        self.e
    }

    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v);
        }
    }

    /// Adds the directed edge v→w to this digraph.
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);

        self.adj[v].push(w);
        self.in_degree[w] += 1;
        self.e += 1;
    }

    /// Returns the vertices adjacent from vertex v.
    pub fn adj(&self, v: usize) -> &Vec<usize> {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns the reverse of the digraph.
    pub fn reverse(&self) -> Digraph {
        let mut r = Digraph::new(self.v);
        for v in 0..self.v {
            for w in self.adj(v).clone() {
                r.add_edge(w, v);
            }
        }
        r
    }

    /// the outdegree of vertex v.
    pub fn out_degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    /// the indegree of vertex v.
    pub fn in_degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.in_degree[v]
    }
}

impl fmt::Display for Digraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} vertices, {} edges", self.v, self.e)?;
        for (i, adj) in self.adj.iter().enumerate() {
            write!(f, "{}: ", i)?;
            for v in adj {
                write!(f, "{} ", v)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_dg() {
        let mut digraph = Digraph::new(13);
        digraph.add_edge(4, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 2);
        digraph.add_edge(6, 0);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 0);
        digraph.add_edge(11, 12);
        digraph.add_edge(12, 9);
        digraph.add_edge(9, 10);
        digraph.add_edge(9, 11);
        digraph.add_edge(8, 9);
        digraph.add_edge(10, 12);
        digraph.add_edge(11, 4);
        digraph.add_edge(4, 3);
        digraph.add_edge(3, 5);
        digraph.add_edge(7, 8);
        digraph.add_edge(8, 7);
        digraph.add_edge(5, 4);
        digraph.add_edge(0, 5);
        digraph.add_edge(6, 4);
        digraph.add_edge(6, 9);
        digraph.add_edge(7, 6);

        assert_eq!(digraph.e(), 22);

        assert_eq!(digraph.in_degree(5), 2);
        assert_eq!(digraph.out_degree(5), 1);

        let mut tmp = digraph.adj(6).clone();
        tmp.sort_unstable();
        assert_eq!(tmp, vec![0, 4, 9]);

        println!("{}", digraph);
    }
}
