pub struct WeightedGraph {
    n_nodes: usize,
    out: Vec<Vec<(usize, i32)>>
}

pub struct FloydShortestPaths {
    w: Vec<Vec<i32>>
}

impl WeightedGraph {
    pub fn new(size: usize) -> WeightedGraph {
        let mut g = WeightedGraph {
            n_nodes: size,
            out: Vec::<Vec<(usize, i32)>>::with_capacity(size)
        };
        for _ in 0..size {
            g.out.push(Vec::<(usize, i32)>::new());
        }
        g
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        assert!(u < self.n_nodes);
        assert!(v < self.n_nodes);
        self.out[u].push((v, weight));
        self.out[v].push((u, weight));
    }

    pub fn get_floyd(&self) -> FloydShortestPaths {
        FloydShortestPaths::compute(self)
    }
}

impl FloydShortestPaths {
    fn compute(graph: &WeightedGraph) -> FloydShortestPaths {
        let mut sp = FloydShortestPaths {
            // Initialize a N x N x N matrix with maxint
            w: vec![vec![i32::max_value(); graph.n_nodes]; graph.n_nodes]
        };
        for u in 0..graph.n_nodes {
            for &(v, weight) in graph.out[u].iter() {
                sp.w[u][v] = weight;
            }
        }
        for k in 0..graph.n_nodes {
            for u in 0..graph.n_nodes {
                for v in 0..graph.n_nodes {
                    if sp.w[u][k] == i32::max_value() || sp.w[k][v] == i32::max_value() {
                        continue;
                    }
                    if sp.w[u][v] > sp.w[u][k] + sp.w[k][v] {
                        sp.w[u][v] = sp.w[u][k] + sp.w[k][v];
                    }

                }
            }
        }
        sp
    }

    pub fn get_shortest_path(&self, u: usize, v: usize) -> i32 {
        self.w[u][v]
    }
}


#[cfg(test)]
mod test {
    use super::WeightedGraph;

    #[test]
    fn basics() {
        let mut g = WeightedGraph::new(10);

        g.add_edge(0, 1, 4);
        g.add_edge(0, 5, 1);
        g.add_edge(1, 2, 4);
        g.add_edge(1, 6, 2);
        g.add_edge(1, 8, 4);
        g.add_edge(2, 3, 5);
        g.add_edge(2, 6, 1);
        g.add_edge(2, 8, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(3, 9, 10);
        g.add_edge(4, 5, 3);
        g.add_edge(4, 6, 5);
        g.add_edge(4, 7, 1);
        g.add_edge(4, 9, 2);
        g.add_edge(5, 6, 1);
        g.add_edge(5, 7, 5);
        g.add_edge(7, 9, 2);
        
    }

    #[test]
    fn all_pairs_shortest_path() {
        let mut g = WeightedGraph::new(10);

        g.add_edge(0, 1, 4);
        g.add_edge(0, 5, 1);
        g.add_edge(1, 2, 4);
        g.add_edge(1, 6, 2);
        g.add_edge(1, 8, 4);
        g.add_edge(2, 3, 5);
        g.add_edge(2, 6, 1);
        g.add_edge(2, 8, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(3, 9, 10);
        g.add_edge(4, 5, 3);
        g.add_edge(4, 6, 5);
        g.add_edge(4, 7, 1);
        g.add_edge(4, 9, 2);
        g.add_edge(5, 6, 1);
        g.add_edge(5, 7, 5);
        g.add_edge(7, 9, 2);

        let floyd = g.get_floyd();

        assert_eq!(floyd.get_shortest_path(0, 1), 4);
        assert_eq!(floyd.get_shortest_path(0, 5), 1);
        assert_eq!(floyd.get_shortest_path(0, 9), 6);
        assert_eq!(floyd.get_shortest_path(0, 6), 2);
        assert_eq!(floyd.get_shortest_path(3, 1), 7);
    }
}
