use std::io;

fn main() {
    ask_user_input();
    let user_weight = get_user_input();
    let mars_weight = compute_weight_in_mars(user_weight);
    show_results_to_user(user_weight, mars_weight);
}

fn compute_weight_in_mars(weight : f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn ask_user_input(){
    println!("Enter your weight in kg with 2 decimal points.")
}

fn get_user_input() -> f32 {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().parse().unwrap()
}

fn show_results_to_user(weight_input : f32, weight_output : f32){
    println!("Your weight on Earth is {}kg, but your weight on Mars is {}kg!", 
            weight_input, 
            weight_output);
}