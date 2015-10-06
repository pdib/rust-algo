pub struct Queue<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>
}

pub struct IntoIter<T>(Queue<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None }
    }

    pub fn enqueue(&mut self, t: T) {
        let new_node = Box::new(Node {
            data: t,
            next: None
        });
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut curr = self.head.as_mut().map(|node| &mut**node);
            while curr.as_mut().and_then(|n| n.next.as_ref()).is_some() {
                curr = curr.and_then(|n| n.next.as_mut().map(|next| &mut**next));
            }
            (*curr.unwrap()).next = Some(new_node);
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let n = *node;
            self.head = n.next;
            n.data
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|next| &**next);
            &node.data
        })        
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn basics() {
        let mut q = Queue::new();
        assert_eq!(q.dequeue(), None);

        q.enqueue(1);
        assert_eq!(q.dequeue(), Some(1));
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), Some(4));
        assert_eq!(q.dequeue(), None);
        q.enqueue(5);
        q.enqueue(6);
        q.enqueue(7);

        assert_eq!(q.dequeue(), Some(5));
        assert_eq!(q.dequeue(), Some(6));
        assert_eq!(q.dequeue(), Some(7));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn iter() {
        let mut q = Queue::new();
        
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);

        let mut i = 1;
        for &k in q.iter() {
            assert_eq!(k, i);
            i = i + 1;
        }
    }

    #[test]
    fn into_iter() {
        let mut q = Queue::new();
        
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);

        let mut i = 1;
        for k in q.into_iter() {
            assert_eq!(k, i);
            i = i + 1;
        }
    }
}
