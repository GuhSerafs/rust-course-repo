pub trait Draw {
    fn draw(&self);

}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32, 
    pub height: u32, 
    pub label: String,    
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
        println!("Drawing a button, width: {}, height {}, label: {}", 
            self.width, 
            self.height,
            self.label
        );
    }
}

// Implementação usando Genéricos
// Desvantagem: 
// A lista de componentes só pode armazenar um tipo T específico 
// do componente que implementa o Draw trait. Se esse fosse o caso, 
// então essa seria a implementação adequada.

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where 
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }