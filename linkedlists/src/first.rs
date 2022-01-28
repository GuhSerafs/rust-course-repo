use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty, 
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    fn pop_node(&mut self) -> Link {
        mem::replace(&mut self.head, Link::Empty)
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = self.pop_node();

        while let Link::More(_) = current_link {
            current_link = self.pop_node();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), None);
    }
}