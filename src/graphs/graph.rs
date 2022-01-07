//! # an undirected graph of vertices named 0 through `V` – 1.
//!
//! This implementation uses an `adjacency-lists` representation.

use std::fmt;
pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(v: usize) -> Graph {
        let mut adj: Vec<Vec<usize>> = Vec::with_capacity(v);
        for _ in 0..v {
            adj.push(Vec::new());
        }
        Graph { v, e: 0, adj }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    fn validate_vertex(&self, vx: usize) {
        if vx >= self.v {
            panic!("vertex cannot >=, {}", self.v);
        }
    }

    /// Adds the undirected edge i-j to this graph
    pub fn add_edge(&mut self, i: usize, j: usize) {
        self.validate_vertex(i);
        self.validate_vertex(j);
        self.e += 1;
        self.adj[i].push(j);
        self.adj[j].push(i);
    }

    /// Returns the vertices adjacent to vertex `i`
    pub fn adj(&self, i: usize) -> &Vec<usize> {
        &self.adj[i]
    }

    /// Returns the degree of vertex `i`
    pub fn degree(&self, i: usize) -> usize {
        self.adj[i].len()
    }
}

impl fmt::Display for Graph {
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
mod tests {
    use super::*;

    #[test]
    fn tiny_graph() {
        let mut graph = Graph::new(13);
        graph.add_edge(0, 5);
        graph.add_edge(4, 3);
        graph.add_edge(0, 1);
        graph.add_edge(9, 12);
        graph.add_edge(6, 4);
        graph.add_edge(5, 4);
        graph.add_edge(0, 2);
        graph.add_edge(11, 12);
        graph.add_edge(9, 10);
        graph.add_edge(0, 6);
        graph.add_edge(7, 8);
        graph.add_edge(9, 11);
        graph.add_edge(5, 3);

        assert_eq!(graph.e(), 13);

        assert_eq!(graph.degree(5), 3);

        let mut tmp = graph.adj(9).clone();
        tmp.sort_unstable();
        assert_eq!(tmp, vec![10, 11, 12]);

        println!("{}", graph);
    }
}
