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

    use utils::cons_list::RefCycleList::{Cons, Nil};

    #[test]
    fn reference_cycle_example() {
        // Criando uma lista com RefCell e Rc's
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        assert_eq!(Rc::strong_count(&a), 1);
        assert!( match **a.tail().unwrap().borrow() {
            Nil => true, 
            _ => false
        });

        // Criando uma segunda lista que aponta pra primeira
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 1);
        assert!( match **b.tail().unwrap().borrow() {
            Cons(5, _) => true, 
            _ => false
        });

        // Criando a refcycle
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b); 
        }

        // Neste ponto, temos b->a->b, então cada Rc aponta para dois endereços.
        assert_eq!(Rc::strong_count(&b), 2);
        assert_eq!(Rc::strong_count(&a), 2);

        // Nesse ponto, chamar a função tail(a) vai criar um stack overflow
        //println!("{:?}", a.tail());
    }

    use utils::Node;
    use std::rc::Weak;

    #[test]
    fn node_weak_smart_pointer_example() {
        let leaf = Rc::new(Node::new (
             3, 
             RefCell::new(Weak::new()), 
             RefCell::new(vec![]),
        ));

        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);

        {
            let branch = Rc::new(Node::new(
                5, 
                RefCell::new(Weak::new()),
                RefCell::new(vec![Rc::clone(&leaf)])
            ));

            // Neste ponto, leaf->leaf e branch->branch
    
            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(Rc::weak_count(&branch), 0);
    
            leaf.set_parent(&branch);
    
            // Neste ponto, leaf->leaf e branch->leaf

            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(Rc::weak_count(&branch), 1);
    
            assert_eq!(Rc::strong_count(&leaf), 2);
            assert_eq!(Rc::weak_count(&leaf), 0);
        }
        // Nesse ponto, mesmo tendo uma soma de 2 apontamentos, como a qtde de 
        // strong points era 1 e branch saiu do escopo, então o valor é dropado
        assert!(match leaf.get_parent_content() {
            None => true, 
            _ => false,
        });

        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);

        // Neste ponto, leaf pode ser dropada
    }
}