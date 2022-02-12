//! # Determining whether a digraph has a directed cycle.
//!
//! This implementation uses depth-first search.

use super::digraph::Digraph;
pub struct DirectedCycle {
    marked: Vec<bool>,   // has vertex v been visited?
    on_stack: Vec<bool>, // is vertex on the stack?
    edge_to: Vec<usize>, // previous vertex on path to v
    cycle: Vec<usize>,   // stack: directed cycle (or empty if no such cycle)
}

impl DirectedCycle {
    pub fn new(g: &Digraph) -> Self {
        let mut dc = DirectedCycle {
            marked: vec![false; g.v()],
            on_stack: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            cycle: vec![],
        };
        for v in 0..g.v() {
            if !dc.marked[v] && dc.cycle.is_empty() {
                dc.dfs(g, v);
            }
        }
        dc
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for w in g.adj(v).clone() {
            // short circuit if directed cycle found
            if !self.cycle.is_empty() {
                return;
            }
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w);
            } else if self.on_stack[w] {
                // trace back directed cycle
                // v -> w -> ... -> v
                let mut x = v;
                while x != w {
                    self.cycle.push(x);
                    x = self.edge_to[x];
                }
                self.cycle.push(w);
                self.cycle.push(v);
            }
        }
        self.on_stack[v] = false;
    }

    /// Does the digraph have a directed cycle?
    pub fn has_cycle(&self) -> bool {
        !self.cycle.is_empty()
    }

    /// Returns a directed cycle if the digraph has a directed cycle.
    /// Note the result relies on adj orders.
    pub fn cycle(&self) -> Iter {
        Iter::new(self)
    }
}

pub struct Iter {
    stack: Vec<usize>,
}

impl Iter {
    pub fn new(dc: &DirectedCycle) -> Self {
        Iter {
            stack: dc.cycle.clone(),
        }
    }
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
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

        let dc = DirectedCycle::new(&digraph);
        assert_eq!(dc.has_cycle(), true);
        assert_eq!(dc.cycle().collect::<Vec<usize>>(), vec![3, 2, 3]);
    }

    #[test]
    fn tiny_dag() {
        let mut dag = Digraph::new(13);
        dag.add_edge(2, 3);
        dag.add_edge(0, 6);
        dag.add_edge(0, 1);
        dag.add_edge(2, 0);
        dag.add_edge(11, 12);
        dag.add_edge(9, 12);
        dag.add_edge(9, 11);
        dag.add_edge(9, 10);
        dag.add_edge(3, 5);
        dag.add_edge(8, 7);
        dag.add_edge(5, 4);
        dag.add_edge(0, 5);
        dag.add_edge(6, 4);
        dag.add_edge(6, 9);
        dag.add_edge(7, 6);

        let dc = DirectedCycle::new(&dag);
        assert_eq!(dc.has_cycle(), false);
        assert_eq!(dc.cycle().collect::<Vec<usize>>(), vec![]);
    }
}
