pub struct CharQueue {
    older: Vec<char>,
    latest: Vec<char>
}

impl CharQueue {
    pub fn new() -> CharQueue {
        CharQueue { older: Vec::new(), latest: Vec::new() }
    }

    pub fn push(&mut self, c: char) {
        self.latest.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.latest.is_empty() {
                return None;
            } else {
                use std::mem::swap;
                swap(&mut self.older, &mut self.latest);
                self.older.reverse();
            }
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.latest.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.latest)
    }
}

#[cfg(test)]
mod tests {
    use super::CharQueue;

    #[test]
    fn empty_queue() {
        let empty_queue = CharQueue::new();
        assert_eq!(empty_queue.is_empty(), true);
    }

    #[test]
    fn pushing_and_poping_chars() {
        let mut queue = CharQueue::new();
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
    fn splitting_the_queue() {
        let mut queue = CharQueue::new();
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
}
