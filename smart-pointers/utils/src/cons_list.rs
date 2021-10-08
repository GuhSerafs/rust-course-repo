use std::{cell::RefCell, rc::{Rc, Weak}};
/*
A implementacao abaixo da erro, pois o compilador nao eh capaz de 
definir o tamanho da lista

enum List {
    Cons(i32, List),
    Nil,
}
*/
#[derive(Debug)]
pub enum BoxList {
    Cons(i32, Box<Self>),
    Nil,
}

pub enum RcList {
    Cons(i32, Rc<Self>), 
    Nil,
}

#[derive(Debug)]
pub enum RefCycleList {
    Cons(i32, RefCell<Rc<Self>>),
    Nil,
}

use RefCycleList::{Cons, Nil};

impl RefCycleList {
    pub fn tail(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil=> None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Weak<Self>>,
    children: RefCell<Vec<Rc<Self>>>,
}

impl Node {
    pub fn new(value: i32, parent: RefCell<Weak<Self>>, children: RefCell<Vec<Rc<Self>>>) -> Self {
        Node { value, parent, children}
    }

    pub fn set_parent(&self, new_parent: &Rc<Node>) -> (){
        *self.parent.borrow_mut() = Rc::downgrade(new_parent);
    }

    pub fn get_parent_content(&self) -> Option<Rc<Self>> {
        self.parent.borrow().upgrade()
    }
        
}

