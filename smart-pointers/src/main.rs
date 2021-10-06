
use utils::List::{Cons, Nil};
use utils::MyBox;

fn main() {
    first_example();
    cons_example();
    deref_example();
    deref_example_with_box();
    deref_example_my_box();
    deref_coercion_example();
}

fn first_example() {
    // b -> heap ao inves do stack
    let b = Box::new(5);
    println!("b = {}", b);
}

fn cons_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
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