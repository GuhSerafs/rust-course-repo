use std::iter::{repeat, once};

fn main() {
    let fizz = repeat("").take(2).chain(once("fizz")).cycle();
    let buzz = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzbuzz= (1..=100).zip(fizz.zip(buzz))
        .map(|tuple| 
            match tuple {
                (i, ("", "")) => i.to_string(),
                (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
            }).collect::<Vec<String>>(); // Incluindo o turbofish apenas por motivos pedagógicos
    for line in fizzbuzz {
        println!("{}", line);
    }
}
