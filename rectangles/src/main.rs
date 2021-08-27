struct Rectangle {
    width: u32, 
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let my_rect = Rectangle{
        width: 30, 
        height: 50
    };
    let mut another_rect = Rectangle{
        width: 27, 
        height: 55
    };

    // Importante: ao passar a struct como uma 
    // referência &, a main() não perde a posse da variável
    println!("The area of the my_rect is {} square pixels!",
             my_rect.area());

    println!("This is {} that my_rect can hold another_rect!",
             my_rect.can_hold(&another_rect));

    another_rect.height = 35;

    println!("With another_rect height changed to {}, now is {} that my_rect can hold another_rect!", 
            another_rect.height,
            my_rect.can_hold(&another_rect));
}

