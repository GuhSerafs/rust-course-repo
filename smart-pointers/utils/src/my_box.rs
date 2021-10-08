use std::{ops::Deref};
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