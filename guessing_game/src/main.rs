use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Mensagem inicial para o usuário
    println!("Guessing game!");
    // Gerando um valor aleatório
    let randval: u32 = rand::thread_rng().gen_range(1..101);
    game_loop(randval);
}

fn game_loop(target: u32) {
    loop {
        println!("Please, input a guess between 1 and 100!");
        // Pegando a entrada do usuário e convertendo para inteiro
        let guess = get_user_input();
        println!("Your guess: {}", guess);
        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

fn get_user_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the user input line.");
    return input.trim().parse().expect("Please, enter a numeric value.");  
}