#[cfg(test)]
mod tests {
    use std::vec;

    fn f(x: Vec<isize>) -> Vec<isize>{
        return x.iter().map(|x| x*x).collect();
    }

    fn g(x: Vec<isize>) -> Vec<isize>{
        return x.iter().map(|x| x*x + 2).collect();
    }

    #[test]
    fn assign_operation_moves_ownership() {
        // Instead, what you should do is to clone 

        let s = vec!["Batatas", "Bananas", "Brocolis"];
        let t = s;
        // let u = s; // Not allowed, because s was moved to t
        let u = t;
        assert_eq!(u, vec!["Batatas", "Bananas", "Brocolis"]);
    }

    #[test]
    fn updating_variable_value_drops_previous_value() {
        let mut s = "Mercedes".to_string();
        let t = s; // Mercedes' ownership goes into t and s becomes uninitialized
        s = "Marimar".to_string(); // This line initializes s again

        assert_eq!(t, "Mercedes".to_string());
        assert_eq!(s, "Marimar".to_string());
    }

    #[test]
    fn passing_variables_to_functions_also_moves_it(){
        let x = vec![10, 20, 30];
        let c = false;
        let ans = if c {
            f(x)
        } else {
            g(x)
        };
        // g(x); // Can't do it, because x was moved into f or g

        assert_eq!(ans, vec![102, 402, 902]);
    }

    #[test]
    fn moves_not_allowed_in_indexed_content() {
        // Indexed content (aka vectors) should have their elements accessed by reference
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        //let third = v[2]; // Can't make like this
        let third = &v[2]; // Allowed
        let fifth = &v[4];

        assert_eq!(*third, 103.to_string());
        assert_eq!(*fifth, 105.to_string());
    }

    #[test]
    fn moves_alternatives_in_indexed_content() {
        // Indexed content (aka vectors) should have their elements accessed by reference
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        // 1. Pop a value off the end of the vector: 
        let last = v.pop().expect("Empty vector!");
        assert_eq!(last, "105");

        // 2. Move a value out of a given index in the vector, 
        // and move the last element into its spot
        let second = v.swap_remove(1);
        assert_eq!(second, "102");
        assert_eq!(v, vec!["101", "104", "103"]);

        let third = std::mem::replace(&mut v[2], "subs".to_string());
        assert_eq!(third, "103");
        assert_eq!(v, vec!["101", "104", "subs"]);
    }
}
