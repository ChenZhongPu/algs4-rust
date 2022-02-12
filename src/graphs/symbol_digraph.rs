//! # A digraph, where the vertex names are arbitrary strings.

use std::collections::HashMap;

use super::digraph::Digraph;
pub struct SymbolDigraph<'a> {
    st: HashMap<&'a str, usize>, // string -> index
    keys: Vec<&'a str>,          // index -> string
    dg: Digraph,                 // underlying digraph
}

impl<'a> SymbolDigraph<'a> {
    pub fn new(data: Vec<&'a str>, delimiter: &str) -> Self {
        let mut st = HashMap::new();
        for &line in &data {
            let a: Vec<&str> = line.split(delimiter).collect();
            for item in a {
                if !st.contains_key(item) {
                    st.insert(item, st.len());
                }
            }
        }

        // inverted index
        let mut keys = vec![""; st.len()];
        for (&k, &v) in &st {
            keys[v] = k;
        }

        // second pass to build graph
        let mut dg = Digraph::new(st.len());
        for line in data {
            let a: Vec<&str> = line.split(delimiter).collect();
            let v = st[a[0]];
            for &name in &a[1..] {
                dg.add_edge(v, st[name]);
            }
        }

        SymbolDigraph { st, keys, dg }
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
        self.keys[v]
    }

    pub fn digraph(&self) -> &Digraph {
        &self.dg
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

        let sg = SymbolDigraph::new(data, " ");

        let graph = sg.digraph();

        if let Some(v) = sg.index_of("JFK") {
            let mut result = graph
                .adj(v)
                .clone()
                .into_iter()
                .map(|w| sg.name_of(w))
                .collect::<Vec<&str>>();
            result.sort_unstable();
            assert_eq!(result, vec!["ATL", "MCO", "ORD"]);
        }
    }
}
