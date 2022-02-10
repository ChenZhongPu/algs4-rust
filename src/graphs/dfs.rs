use crate::graphs::graph::Graph;

pub struct DepthFirstSearch {
    marked: Vec<bool>,
    count: usize,
    source: usize,
}

impl DepthFirstSearch {
    pub fn new(g: &Graph, s: usize) -> DepthFirstSearch {
        let mut df_search = DepthFirstSearch {
            marked: vec![false; g.v()],
            count: 0,
            source: s,
        };
        df_search.dfs(g);
        df_search
    }

    fn dfs(&mut self, g: &Graph) {
        self._dfs(g, self.source);
    }

    fn _dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.count += 1;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self._dfs(g, w);
            }
        }
    }

    pub fn marked(&self, w: usize) -> bool {
        self.marked[w]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs() {
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

        let search = DepthFirstSearch::new(&graph, 0);
        assert_eq!(search.count(), 7);

        let search = DepthFirstSearch::new(&graph, 9);
        assert_eq!(search.count(), 4);
    }
}
