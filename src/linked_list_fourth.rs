use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    data: T
}

pub struct IntoIter<T>(List<T>);


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(head) => {
                    head.borrow_mut().prev.take();
                    self.head = Some(head);
                }
                None => {
                    self.tail.take();
                }
            };
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            };
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|head| {
            Ref::map(head.borrow() , |head_borrow| &head_borrow.data)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|tail| {
            Ref::map(tail.borrow(), |tail_borrow| &tail_borrow.data)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|tail| {
            RefMut::map(tail.borrow_mut(), |tail_borrow| &mut tail_borrow.data)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|head| {
            RefMut::map(head.borrow_mut(), |head_borrow| &mut head_borrow.data)
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { data: data, prev: None, next: None }))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option <T> {
        self.0.pop_back()
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics_front() {
        let mut l = List::<i32>::new();
        assert_eq!(l.pop_front(), None);
        
        l.push_front(5);
        l.push_front(4);
        
        assert_eq!(l.pop_front(), Some(4));
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), None);

        l.push_front(5);
        l.push_front(4);
        
        assert_eq!(&*l.peek_front().unwrap(), &4);
        assert_eq!(l.pop_front(), Some(4));
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), None);
    }

    #[test]
    fn basics_back() {
        let mut l = List::<i32>::new();
        assert_eq!(l.pop_back(), None);
        
        l.push_back(5);
        l.push_back(4);

        assert_eq!(&*l.peek_back().unwrap(), &4);
        assert_eq!(l.pop_back(), Some(4));
        assert_eq!(l.pop_back(), Some(5));
        assert_eq!(l.pop_back(), None);

        l.push_back(5);
        l.push_back(4);

        assert_eq!(l.pop_back(), Some(4));
        assert_eq!(l.pop_back(), Some(5));
        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn basics_front_back() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        l.push_front(0);

        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.pop_front(), Some(0));
        assert_eq!(l.pop_front(), Some(1));
    }

    #[test]
    fn basics_mut() {
        let mut l = List::new();

        l.push_back(1);
        l.push_back(2);
        {
            let mut k = l.peek_front_mut().unwrap();
            *k = 5;
        }
        {
            let mut k = l.peek_back_mut().unwrap();
            *k = 8;
        }
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), Some(8));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
