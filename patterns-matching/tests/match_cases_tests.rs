#[cfg(test)]
mod tests {

    #[test]
    fn match_arms() {
        enum Language {
            English,
            Spanish,
            Portuguese,
            Japanese,
        }

        let language = Language::Portuguese;
        let mut message = String::new();

        match language {
            Language::English => message.push_str("Hello world!"), 
            Language::Spanish => message.push_str("Hola mundo!"), 
            Language::Portuguese => message.push_str("Olá mundo!"), 
            _ => message.push_str("Unsuported Language!"), 
        }

        assert_eq!(message, "Olá mundo!");
    }

    #[test]
    fn conditional_if_let() {
        let auth_status: Option<&str> = None;
        let is_admin = false;
        let group_id: Result<u8, _> = "34".parse();

        let mut auth_msg = String::new();

        if let Some(status) = auth_status {
            auth_msg.push_str(&format!("Authorization status: {}", status));
        } else if is_admin {
            auth_msg.push_str("Authorization status: admin");
        } else if let Ok(group_id) = group_id {
            if group_id > 30 {
                auth_msg.push_str("Authorization status: priviledged");
            } else {
                auth_msg.push_str("Authorization status: basic");
            }
        }

        assert_eq!(auth_msg, "Authorization status: priviledged");
    }

    #[test]
    fn conditional_while_let() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut value = 3;
        while let Some(top) = stack.pop() {
            assert_eq!(value, top);
            value-=1;
        }
    }

    #[test]
    fn let_statements() {
        let (x, y, _) = (32, 40, 99);

        assert_eq!(x, 32);
        assert_eq!(y, 40);
    }

    #[test]
    fn parameters() {
        fn print_coordinates(&(x, y): &(i32, i32)) -> String {
            format!("Current location: ({}, {})", x, y)
        }

        let point = (3, 5);
        let location = print_coordinates(&point);
        assert_eq!(location, "Current location: (3, 5)");
    }

    #[test]
    fn literals() {
        let x = 1;
        let mut result = String::new();
        match x {
            1 => result.push_str("One"), 
            2 => result.push_str("Two"), 
            3 => result.push_str("Three"), 
            _ => result.push_str("Any")
        }

        assert_eq!(result, "One");
    }

    
    #[test]
    #[allow(unused)]
    fn named_variables() {
        let x = Some(5);
        let y = 10;
        let mut result = 0;

        match x {
            Some(50) => result = 50,
            Some(y) => result = y, // Este y no escopo do match obscurece o y da função
            _ => result = -1,
        }

        assert_eq!(result, 5);
    }

    #[test]
    #[allow(unused)]
    fn multiple_matching() {
        let x = 1;
        let mut res = 0;
        let mut range = false;
        
        // Valores múltiplos
        match x {
            1 | 3 | 5 => res = 10, 
            2 | 4 => res = 8,
            _ => res = -1 
        }

        // Faixa de valores
        match res {
            0..=10 => range = true, 
            _ => range = false,
        }

        assert_eq!(res, 10);
        assert!(range)
    }

    #[test]
    fn struct_unpacking_as_they_say_in_python() {
        struct Point {
            x: i32, 
            y: i32,
        }
        let mut is_point_in_x_axis = false;
        let mut is_point_in_y_axis = false;

        let p = Point {x:0, y: 7};

        match p { 
            Point {x: _, y: 0} => is_point_in_x_axis = true, 
            Point {x: 0, y: _} => is_point_in_y_axis = true,
            _ => (),
        }

        assert!(!is_point_in_x_axis);
        assert!(is_point_in_y_axis);
    }

    #[test]
    fn enum_unpacking_as_they_say_in_python() {
        enum Color {
            Rgb{r: u8, g: u8, b: u8}, 
            Hsv{h: u8, s: u8, v: u8},
        }
        enum Message {
            Quit, 
            Move{x: i32, y: i32}, 
            Write(String), 
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Rgb{r: 0, g: 160, b: 255});

        let mut action = String::new();

        match msg {
            Message::Quit => {
                action.push_str("Quit");
            }, 
            Message::Move {x, y} => {
                action.push_str(&format!("Move to x: {}, y: {}", x, y));
            },
            Message::Write(text) => {
                action.push_str(&format!("Message: {}", text));
            }
            Message::ChangeColor(Color::Rgb{r, g, b}) => {
                action.push_str(&format!("Change color: red {}, green {}, blue {}.", r, g, b));
            }
            Message::ChangeColor(Color::Hsv{h, s, v}) => {
                action.push_str(&format!("Change color: hue {}, saturation {}, value {}.", h, s, v));
            }
        }

        assert_eq!(action, "Change color: red 0, green 160, blue 255.");
    }

    #[test]
    fn mixed_unpacking() {
        struct Point {
            x: i32, 
            y: i32,
        }

        let ((feet, inches), Point {x, y}) = ((3, 10), Point{x: 3, y: 10});
        assert_eq!(feet, 3);
        assert_eq!(inches, 10);
        assert_eq!(x, 3);
        assert_eq!(y, 10);
    }
}
