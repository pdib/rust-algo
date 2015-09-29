pub struct UndirectedGraph<T> {
    edges: Vec<Vec<usize>>,
    data: Vec<Option<T>>
}

pub struct DfsIterator<'a, T: 'a> {
    graph: &'a UndirectedGraph<T>,
    stack: Vec<usize>,
    visited: Vec<bool>
}

pub struct BfsIterator<'a, T: 'a> {
    graph: &'a UndirectedGraph<T>,
    queue: Vec<usize>,
    visited: Vec<bool>
}

impl<'a, T: Clone> BfsIterator<'a, T> {
    pub fn new(graph: &'a UndirectedGraph<T>, start: usize) -> BfsIterator<'a, T> {
        let size = graph.size();
        let mut it = BfsIterator {
            graph: graph,
            queue: vec![start],
            visited: Vec::<bool>::with_capacity(size)
        };
        for i in 0..size {
            it.visited.push(i == start);
        }
        it
    }
}

impl<'a, T: Clone> Iterator for BfsIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            let u = self.queue.remove(0);
            for v in self.graph.from(u) {
                if !self.visited[*v] {
                    self.visited[*v] = true;
                    self.queue.push(*v);
                }
            }
            match self.graph.get_data(u) {
                Some(t) => Some(t.clone()),
                _ => None
            }
        }
    }
}


impl<'a, T: Clone> DfsIterator<'a, T> {
    pub fn new(graph: &'a UndirectedGraph<T>, start: usize) -> DfsIterator<'a, T> {
        let size = graph.size();
        let mut it = DfsIterator {
            graph: graph,
            stack: vec![start],
            visited: Vec::with_capacity(size)
        };
        for i in 0..size {
            it.visited.push(i == start);
        }
        it
    }
}

impl<'a, T: Clone> Iterator for DfsIterator<'a, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<T> {
        if self.stack.is_empty() {
            None
        } else {
            let mut next_node = self.stack[self.stack.len() - 1];
            let mut found = true;
            
            while found {
                found = false;
                for v in self.graph.from(next_node) {
                    if !self.visited[*v] {
                        found = true;
                        next_node = *v;
                        self.visited[*v] = true;
                        self.stack.push(*v);
                        break;
                    }
                }
            }
            self.stack.pop();
            match self.graph.get_data(next_node) {
                Some(t) => Some(t.clone()),
                _ => None
            }
        }
    }
}

impl<T> UndirectedGraph<T> {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
        self.edges[v].push(u);
    }
    
    pub fn from(&self, u: usize) -> &Vec<usize> {
        &self.edges[u]
    }
}


impl<T: Clone> UndirectedGraph<T> {
    pub fn new(size: usize) -> UndirectedGraph<T> {
        let mut s = UndirectedGraph {
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
        if u < self.data.len() {
            self.data[u] = Some(t);
        }
    }

    pub fn get_data(&self, u: usize) -> Option<&T> {
        if u < self.data.len() {
            self.data[u].as_ref()
        } else {
            None
        }

    }

    pub fn dfs(&mut self, start: usize) -> DfsIterator<T> {
        DfsIterator::new(self, start)
    }

    pub fn bfs(&mut self, start: usize) -> BfsIterator<T> {
        BfsIterator::new(self, start)
    }
}

#[cfg(test)]
mod test {
    use super::UndirectedGraph;

    #[test]
    fn simple_graph () {
        let mut s = UndirectedGraph::<i32>::new(3);
        s.add_edge(0, 1);
        s.add_edge(1, 2);
        assert_eq!(s.from(0), &[1]);
        assert_eq!(s.from(1), &[0, 2]);
        assert_eq!(s.from(2), &[1]);
    }

    #[test]
    fn size_non_zero() {
        let s = UndirectedGraph::<i32>::new(10);
        assert_eq!(s.size(), 10);
    }

    #[test]
    fn size_zero() {
        let s = UndirectedGraph::<i32>::new(0);
        assert_eq!(s.size(), 0);
    }
    
    #[test]
    fn with_data () {
        let mut s = UndirectedGraph::<i32>::new(2);
        s.set_data(0, 1337);
        assert_eq!(*s.get_data(0).unwrap(), 1337);
        assert!(s.get_data(1).is_none());
    }


    #[test]
    fn depth_first_search () {
        let mut s = UndirectedGraph::<i32>::new(5);
        s.set_data(0, 0);
        s.set_data(1, 1);
        s.set_data(2, 2);
        s.set_data(3, 3);
        s.set_data(4, 4);

        s.add_edge(0, 1);
        s.add_edge(0, 2);

        s.add_edge(1, 3);
        s.add_edge(3, 4);


        assert_eq!(s.dfs(2).collect::<Vec<i32>>(), vec![4, 3, 1, 0, 2]);
        assert_eq!(s.dfs(1).collect::<Vec<i32>>(), vec![2, 0, 4, 3, 1]);
    }

    
    #[test]
    fn breadth_first_search () {
        let mut s = UndirectedGraph::<i32>::new(5);
        s.set_data(0, 0);
        s.set_data(1, 1);
        s.set_data(2, 2);
        s.set_data(3, 3);
        s.set_data(4, 4);

        s.add_edge(0, 1);
        s.add_edge(0, 2);

        s.add_edge(1, 3);
        s.add_edge(3, 4);


        assert_eq!(s.bfs(2).collect::<Vec<i32>>(), vec![2, 0, 1, 3, 4]);
        assert_eq!(s.bfs(1).collect::<Vec<i32>>(), vec![1, 0, 3, 2, 4]);
    }
}

