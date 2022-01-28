#[cfg(test)]
mod vector_tests {

    #[test]
    fn bin_heap() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::from(vec![2, 3, 7, 5, 9, 6, 1]);
        assert_eq!(heap.peek(), Some(&9));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(7));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(5));
    }   
}