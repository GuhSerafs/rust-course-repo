use std::{ops::Deref, rc::Rc};


/*
A implementacao abaixo da erro, pois o compilador nao eh capaz de 
definir o tamanho da lista

enum List {
    Cons(i32, List),
    Nil,
}
*/

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum RcList {
    Cons(i32, Rc<RcList>), 
    Nil,
}

pub struct MyBox<T>(T); // Implementaçao da minha propria box

// Implementaçao do new
impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementacao da trait "deref"
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self){
        println!("Saindo de escopo!");
    }
}