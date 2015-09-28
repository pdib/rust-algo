struct BinaryHeap {
    array: Vec<i32>
}

impl BinaryHeap {
    pub fn new() -> BinaryHeap {
        BinaryHeap {
            array: Vec::<i32>::new()
        }
    }

    pub fn get_min(&self) -> i32 {
        if self.array.len() == 0 {
            panic!("get_min called on empty heap");
        }
        self.array[0]
    }

    pub fn insert(&mut self, x: i32) {
        self.array.push(x);
        let idx = self.array.len() - 1;
        self.bubble_up(idx);
    }

    pub fn remove_min(&mut self) {
        if self.array.len() == 0 {
            panic!("Remove called on empty heap");
        }
        self.array.swap_remove(0); // replace first element with last
        self.bubble_down(0);
    }

    fn bubble_up (&mut self, index: usize) {
        if index <= 0 {
            return;
        }
        if self.array[index] < self.array[BinaryHeap::get_parent(index)] {
            self.array.swap(index, BinaryHeap::get_parent(index));
            self.bubble_up(BinaryHeap::get_parent(index));
        }
    }

    fn bubble_down (&mut self, index: usize) {
        let fc = BinaryHeap::get_first_child(index);
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

        assert_eq!(h.array[0], h.get_min());
        assert_eq!(h.get_min(), -6);
    }

    #[test]
    #[should_panic(expected = "empty")]
    fn get_min_empty_heap () {
        let mut h = BinaryHeap::new();
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
        let mut h = BinaryHeap::new();
        h.remove_min();
    }
}


