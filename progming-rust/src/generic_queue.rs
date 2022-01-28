pub struct GenericQueue<T> {
    older: Vec<T>,
    latest: Vec<T>
}

impl<T> GenericQueue<T> {
    pub fn new() -> GenericQueue<T> {
        GenericQueue { older: Vec::new(), latest: Vec::new() }
    }

    pub fn push(&mut self, c: T) {
        self.latest.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.latest.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.latest);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.latest.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.latest)
    }
}

#[cfg(test)]
mod tests {
    use super::GenericQueue;

    #[test]
    fn empty_queue() {
        let empty_queue: GenericQueue<char> = GenericQueue::new();
        assert_eq!(empty_queue.is_empty(), true);
    }

    #[test]
    fn pushing_and_poping_chars() {
        let mut queue = GenericQueue::new();
        queue.push('C');
        queue.push('O');
        queue.push('o');
        queue.push('L');
        assert_eq!(queue.pop(), Some('C'));
        assert_eq!(queue.pop(), Some('O'));
        assert_eq!(queue.pop(), Some('o'));
        assert_eq!(queue.pop(), Some('L'));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn splitting_the_chars_queue() {
        let mut queue = GenericQueue::new();
        queue.push('A');
        queue.push('b');
        queue.push('C');
        queue.push('d');
        assert_eq!(queue.pop(), Some('A'));
        queue.push('e');
        queue.push('F');
        queue.push('g');
        queue.push('H');
        assert_eq!(queue.split(), (vec!['d', 'C', 'b'], vec!['e', 'F', 'g', 'H']));
    }

    #[test]
    fn pushing_and_poping_strings() {
        let mut queue = GenericQueue::new();
        queue.push("Carlos");
        queue.push("Olinda");
        queue.push("Romulo");
        queue.push("Lucas");
        assert_eq!(queue.pop(), Some("Carlos"));
        assert_eq!(queue.pop(), Some("Olinda"));
        assert_eq!(queue.pop(), Some("Romulo"));
        assert_eq!(queue.pop(), Some("Lucas"));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn splitting_the_floats_queue() {
        let mut queue = GenericQueue::new();
        queue.push(1.07);
        queue.push(4.98);
        queue.push(3.25);
        queue.push(0.12);
        assert_eq!(queue.pop(), Some(1.07));
        queue.push(6.00);
        queue.push(2.11);
        queue.push(5 as f64);
        queue.push(0.00);
        assert_eq!(queue.split(), (vec![0.12, 3.25, 4.98], vec![6.00, 2.11, 5 as f64, 0.00]));
    }
}
