pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    data: T
}

impl<T> List<T> {
    pub fn new () -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push (&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take()
        });
        self.head = Some(new_node)
    }

    pub fn pop (&mut self) -> Option<T> {
        self.head.take().map(|boxed| {
            let node = *boxed;
            self.head = node.next;
            node.data
        })
    }

    pub fn peek (&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }
}


impl<T> Drop for List<T> {
    fn drop (&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            // Replacing boxed_node.next with Link::Empty avoids the
            // unbound .drop() recursion.
            // boxed_node is dropped at the end of this scope.
            current_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    ptr: Option<&'a Node<T>>
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { ptr: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|node| {
            self.ptr = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}


pub struct IterMut<'a, T: 'a> {
    ptr: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { ptr: self.head.as_mut().map(|node| &mut**node) }
    }
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|node| {
            self.ptr = node.next.as_mut().map(|node| &mut**node);
            &mut node.data
        })
    }
}



#[cfg(test)]
mod test {
    use super::List;
    
    #[test]
    fn create_list () {
        List::<i32>::new();
    }

    #[test]
    fn push () {
        let mut l = List::new();
        l.push(2);
        assert_eq!(l.pop(), Some(2));
    }

    #[test]
    fn peek () {
        let mut l = List::new();
        l.push(2);
        assert_eq!(l.peek(), Some(&2));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.peek(), None);
    }
    
    #[test]
    fn push_and_pop () {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
        l.push(3);
        assert_eq!(l.pop(), Some(3));
    }

    #[test]
    fn into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        l.push(4);
        let mut i = 4;
        for j in l.into_iter() {
            assert_eq!(j, i);
            i = i - 1;
        }
    }

    #[test]
    fn iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        l.push(4);
        let mut i = 4;
        for &j in l.iter() {
            assert_eq!(j, i);
            i = i - 1;
        }
        l.push(5);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        {
            let mut iter = list.iter_mut();

            if let Some(res) = iter.next() {
                assert_eq!(res, &mut 3);
                *res = 7; // modifying the first element from the IterMut
            } else {
                // Should not reach
                assert!(false);
            }
            
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 1));
        } // Mutable references are released here
        assert_eq!(list.peek(), Some(&7));
    }
}
