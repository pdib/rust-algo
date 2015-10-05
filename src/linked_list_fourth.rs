use std::rc::Rc;
use std::cell::RefCell;

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
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { data: data, prev: None, next: None }))
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut l = List::<i32>::new();
        assert_eq!(l.pop_front(), None);
        
        l.push_front(5);
        l.push_front(4);
        
        assert_eq!(l.pop_front(), Some(4));
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), None);

        l.push_front(5);
        l.push_front(4);

        assert_eq!(l.pop_front(), Some(4));
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), None);
    }
}
