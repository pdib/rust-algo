use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    next: Link,
    data: i32
}

enum Link {
    More(Box<Node>),
    Empty
}

impl List {
    pub fn new () -> Self {
        List { head: Link::Empty }
    }

    pub fn push (&mut self, value: i32) {
        let new_node = Box::new(Node {
            data: value,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node)
    }

    pub fn pop (&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(boxed) => {
                let node = *boxed;
                self.head = node.next;
                Some(node.data)
            },
            Link::Empty => None
        }
    }
}


impl Drop for List {
    fn drop (&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current_link {
            // Replacing boxed_node.next with Link::Empty avoids the
            // unbound .drop() recursion.
            // boxed_node is dropped at the end of this scope.
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    
    #[test]
    fn create_list () {
        List::new();
    }

    #[test]
    fn push () {
        let mut l = List::new();
        l.push(2);
        assert_eq!(l.pop(), Some(2));
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
}
