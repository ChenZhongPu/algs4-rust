//! # An undirected graph, where the vertex names are arbitrary strings.

use std::collections::HashMap;

use super::graph::Graph;
pub struct SymbolGraph<'a> {
    st: HashMap<&'a str, usize>, // string -> index
    keys: Vec<&'a str>,          // index -> string
    graph: Graph,                // the underlying graph
}

impl<'a> SymbolGraph<'a> {
    pub fn new(data: Vec<&'a str>, delimiter: &str) -> SymbolGraph<'a> {
        let mut st = HashMap::new();
        for &line in &data {
            let a: Vec<&str> = line.split(delimiter).collect();
            for item in a {
                if !st.contains_key(item) {
                    st.insert(item, st.len());
                }
            }
        }

        // inverted index to get string keys in a vector
        let mut keys = vec![""; st.len()];
        for (&k, &v) in &st {
            keys[v] = k;
        }

        // second pass to build graph
        let mut graph = Graph::new(st.len());
        for line in data {
            let a: Vec<&str> = line.split(delimiter).collect();
            let v = st[a[0]];
            for &name in &a[1..] {
                graph.add_edge(v, st[name]);
            }
        }

        SymbolGraph { st, keys, graph }
    }

    /// Does the graph contain the vertex named `s`?
    pub fn contains(&self, s: &str) -> bool {
        self.st.contains_key(s)
    }

    /// Returns the integer associated with the vertex named `s`.
    pub fn index_of(&self, s: &str) -> Option<usize> {
        self.st.get(s).copied()
    }

    /// Returns the name of the vertex associated with the integer `v`
    pub fn name_of(&self, v: usize) -> &str {
        self.validate_vertex(v);
        self.keys[v]
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    fn validate_vertex(&self, v: usize) {
        if v >= self.graph.v() {
            panic!("vertex {} is not between 0 and {}", v, self.graph.v());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn routes() {
        let data = vec![
            "JFK MCO", "ORD DEN", "ORD HOU", "DFW PHX", "JFK ATL", "ORD DFW", "ORD PHX", "ATL HOU",
            "DEN PHX", "PHX LAX", "JFK ORD", "DEN LAS", "DFW HOU", "ORD ATL", "LAS LAX", "ATL MCO",
            "HOU MCO", "LAS PHX",
        ];

        let sg = SymbolGraph::new(data, " ");
        assert_eq!(sg.contains("JFK"), true);
        let graph = sg.graph();

        let mut adjs = Vec::new();
        if let Some(s) = sg.index_of("JFK") {
            for v in graph.adj(s).clone() {
                adjs.push(sg.name_of(v));
            }
        }
        adjs.sort_unstable();
        assert_eq!(adjs, vec!["ATL", "MCO", "ORD"]);

        assert_eq!(sg.contains("LAB"), false);
        let mut adjs = Vec::new();
        if let Some(s) = sg.index_of("LAX") {
            for v in graph.adj(s).clone() {
                adjs.push(sg.name_of(v));
            }
        }
        adjs.sort_unstable();
        assert_eq!(adjs, vec!["LAS", "PHX"]);
    }
}
