pub struct BinaryHeap<T> {
    array: Vec<T>
}

pub fn heapsort<T>(v: &mut Vec<T>) where T: Ord {
    let mut heap = BinaryHeap::<T>::new();
    while !v.is_empty() {
        heap.insert(v.pop().unwrap());
    }
    while !heap.is_empty() {
        v.push(heap.remove_min());
    }
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap::<T> {
            array: Vec::<T>::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
    
    pub fn get_min(&self) -> &T {
        if self.array.len() == 0 {
            panic!("get_min called on empty heap");
        }
        &self.array[0]
    }

    pub fn insert(&mut self, x: T) {
        self.array.push(x);
        let idx = self.array.len() - 1;
        self.bubble_up(idx);
    }

    pub fn remove_min(&mut self) -> T {
        if self.array.len() == 0 {
            panic!("Remove called on empty heap");
        }
        let res = self.array.swap_remove(0); // replace first element with last
        self.bubble_down(0);
        res
    }

    fn bubble_up (&mut self, index: usize) {
        if index <= 0 {
            return;
        }
        let parent = BinaryHeap::<T>::get_parent(index);
        if self.array[index] < self.array[parent] {
            self.array.swap(index, parent);
            self.bubble_up(parent);
        }
    }

    fn bubble_down (&mut self, index: usize) {
        let fc = BinaryHeap::<T>::get_first_child(index);
        if fc >= self.array.len() {
            return;
        }
        let min_idx = if fc + 1 >= self.array.len() || self.array[fc] < self.array[fc + 1] {
            fc
        } else {
            fc + 1
        };
        if self.array[index] <= self.array[min_idx] {
            return;
        }
        self.array.swap(index, min_idx);
        self.bubble_down(min_idx);
    }

    fn get_parent (index: usize) -> usize {
        (index - 1) / 2
    }

    fn get_first_child (index: usize) -> usize {
        index * 2 + 1
    }
}

#[cfg(test)]
mod test {
    use super::BinaryHeap;
    use super::heapsort;   

    #[test]
    fn insert () {
        let mut h = BinaryHeap::new();
        h.insert(5);
        h.insert(4);
        h.insert(6);
        h.insert(3);

        assert_eq!(h.array[0], 3);
    }

    #[test]
    fn get_min () {
        let mut h = BinaryHeap::new();
        h.insert(-1);
        h.insert(4);
        h.insert(-6);
        h.insert(3);

        assert_eq!(h.array[0], *h.get_min());
        assert_eq!(*h.get_min(), -6);
    }

    #[test]
    #[should_panic(expected = "empty")]
    fn get_min_empty_heap () {
        let h = BinaryHeap::<i32>::new();
        h.get_min();
    }

    #[test]
    fn remove_min () {
        let mut h = BinaryHeap::new();
        h.insert(5);
        h.insert(4);
        h.insert(6);
        h.insert(3);
        h.remove_min();
        
        assert_eq!(h.array[0], 4);
        assert_eq!(h.array.len(), 3);
    }

    #[test]
    fn heap_sorting () {
        let mut h = BinaryHeap::new();
        h.insert(5);
        h.insert(4);
        h.insert(6);
        h.insert(3);

        assert_eq!(h.array[0], 3);
        h.remove_min();
        assert_eq!(h.array[0], 4);        
        h.remove_min();
        assert_eq!(h.array[0], 5);
        h.remove_min();
        assert_eq!(h.array[0], 6);
        h.remove_min();
    }

    #[test]
    #[should_panic(expected = "empty")]
    fn remove_on_empty () {
        let mut h = BinaryHeap::<i32>::new();
        h.remove_min();
    }

    #[test]
    fn is_empty () {
        let mut h = BinaryHeap::<i32>::new();
        assert!(h.is_empty());
        h.insert(6);
        assert!(!h.is_empty());
        h.remove_min();
        assert!(h.is_empty());
    }

    #[test]
    fn heapsort_vector () {
        let mut v = vec![3,4,-1,0,192,20];
        heapsort(&mut v);
        assert_eq!(v, [-1, 0, 3, 4, 20, 192]);
    }
}

