struct SimpleGraph<T> {
    edges: Vec<Vec<usize>>,
    data: Vec<Option<T>>
}

impl<T:Clone> SimpleGraph<T> {
    pub fn new(size: usize) -> SimpleGraph<T> {
        let mut s = SimpleGraph {
            edges: Vec::with_capacity(size),
            data: Vec::with_capacity(size)
        };
        for _ in 0..size {
            s.edges.push(Vec::new());
            s.data.push(None);
        }
        s
    }

    pub fn set_data(&mut self, u: usize, t: T) {
        self.data[u] = Some(t);
    }
    
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
        self.edges[v].push(u);
    }
    
    pub fn from(&self, u: usize) -> &Vec<usize> {
        &self.edges[u]
    }
    
    pub fn dfs(&self, start: usize) {
    }
}

#[test]
fn test_simple_graph () {
    let mut s = SimpleGraph::<i32>::new(3);
    s.add_edge(0, 1);
    s.add_edge(1, 2);
    assert_eq!(s.from(0), &[1]);
    assert_eq!(s.from(1), &[0, 2]);
    assert_eq!(s.from(2), &[1]);
}
