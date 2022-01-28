use std::mem;

pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None}
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::None),
        });
        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter{ next: self.head.as_deref() }
    }

    fn pop_node(&mut self) -> Link<T> {
        self.head.take()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.pop_node();

        while let Link::Some(_) = current_link {
            current_link = self.pop_node();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push("Joseph"); list.push("Jotaro"); list.push("Josuke");

        assert_eq!(list.peek(), Some(&"Josuke"));
        assert_eq!(list.peek_mut(), Some(&mut "Josuke"));

        list.peek_mut().map(|value| {
            *value = "Giorno"
        });

        assert_eq!(list.peek(), Some(&"Giorno"));
        assert_eq!(list.pop(), Some("Giorno"));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push('B'); list.push('Q'); list.push('A');

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some('A'));
        assert_eq!(iter.next(), Some('Q'));
        assert_eq!(iter.next(), Some('B'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new(); 
        list.push(1.7); list.push(6.2); list.push(4.9);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&4.9));
        assert_eq!(iter.next(), Some(&6.2));
        assert_eq!(iter.next(), Some(&1.7));
        assert_eq!(iter.next(), None);
    }
}