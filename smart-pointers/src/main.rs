
use std::rc::Rc;

use utils::List::{Cons, Nil};
use utils::RcList::{Cons as RcCons, Nil as RcNil};
use utils::MyBox;

fn main() {
    first_example();
    cons_example();
    deref_example();
    deref_example_with_box();
    deref_example_my_box();
    deref_coercion_example();
    multiple_ownership_problem();
    reference_counter_pointer_example();
}

fn first_example() {
    // b -> heap ao inves do stack
    let b = Box::new(5);
    println!("b = {}", b);
}

fn cons_example() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn deref_example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(x, y);
}

fn deref_example_with_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_example_my_box(){
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_coercion_example() {
    let m = MyBox::new(String::from("Rust"));
    // drop(m); Valor eh desconstruido aqui
    hello(&m); // &MyBox<String> -> &String -> &str
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn multiple_ownership_problem() {
    /* Este snippet nao compila */
    // let a = Cons(5, Box::new(Cons(10, Box::new(Cons(12, Box::new(Nil))))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}

fn reference_counter_pointer_example(){
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcCons(12, Rc::new(RcNil)))))));
    print!("Contagem de referencias apos criar a = {}\n", Rc::strong_count(&a));
    let _b = RcCons(3, Rc::clone(&a));
    print!("Contagem de referencias apos criar b = {}\n", Rc::strong_count(&a));
    {
        let _c = RcCons(4, Rc::clone(&a));
        print!("Contagem de referencias apos criar c = {}\n", Rc::strong_count(&a));
    }
    print!("Contagem de referencias apos dropar c = {}\n", Rc::strong_count(&a));
    
}