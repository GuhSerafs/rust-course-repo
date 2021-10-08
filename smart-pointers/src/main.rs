use utils::List::{Cons, Nil};

fn main() {
    first_example();
    cons_example();
    multiple_ownership_problem();
}

fn first_example() {
    // b -> heap ao inves do stack
    let b = Box::new(5);
    println!("b = {}", b);
}

fn cons_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

fn multiple_ownership_problem() {
    /* Este snippet nao compila */
    // let a = Cons(5, Box::new(Cons(10, Box::new(Cons(12, Box::new(Nil))))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}