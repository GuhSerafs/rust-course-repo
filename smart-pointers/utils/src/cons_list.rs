use std::rc::Rc;
/*
A implementacao abaixo da erro, pois o compilador nao eh capaz de 
definir o tamanho da lista

enum List {
    Cons(i32, List),
    Nil,
}
*/
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum RcList {
    Cons(i32, Rc<RcList>), 
    Nil,
}