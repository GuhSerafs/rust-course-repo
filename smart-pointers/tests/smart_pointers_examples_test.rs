// Testes relacionados com Rc(), RefCell e outros...

#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use utils::RcList::{Cons as RcCons, Nil as RcNil};

    #[test]
    fn reference_counter_pointer_example(){
        let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcCons(12, Rc::new(RcNil)))))));
        assert_eq!(Rc::strong_count(&a), 1);

        let _b = RcCons(3, Rc::clone(&a));
        assert_eq!(Rc::strong_count(&a), 2);
        
        {
            let _c = RcCons(4, Rc::clone(&a));
            assert_eq!(Rc::strong_count(&a), 3);
        }
        
        assert_eq!(Rc::strong_count(&a), 2);

        drop(_b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    use utils::{Messenger, LimitTracker};
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }

    #[test]
    fn over_75_percent_warning_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}