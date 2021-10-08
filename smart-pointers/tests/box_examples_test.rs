#[cfg(test)]
mod tests {

    use utils::MyBox;

    #[test]
    fn deref_example() {
        let x = 5;
        let y = &x;
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // assert_eq!(x, y);
    }
    
    #[test]
    fn deref_example_with_box() {
        let x = 5;
        let y = Box::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    
    #[test]
    fn deref_example_my_box(){
        let x = 5;
        let y = MyBox::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    fn hello(name: &str) -> String {
        format!("Hello {}", name)
    }

    #[test]
    fn deref_coercion_example() {
        let m = MyBox::new(String::from("Rust"));
        // drop(m); Valor eh desconstruido aqui
        // &MyBox<String> -> &String -> &str
        assert_eq!(hello(&m), "Hello Rust");
    }
    
    
}