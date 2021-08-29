// use std::collections
use utils::vect_statistics;

fn main() {
    let v: Vec<isize> = [8, 4, 5, 4, 3, 2, 1, 0].to_vec();

    println!("Média: {}", vect_statistics::average(v))
}