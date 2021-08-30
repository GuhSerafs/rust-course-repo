// use std::collections
use utils::vect_statistics;

fn main() {
    let v: Vec<isize> = [1, 2, 3, 4, 5, 6, 7].to_vec();

    println!("Media: {}", vect_statistics::average(&v));
    println!("Mediana: {}", vect_statistics::median(&v));
    println!("Moda: {:?}", vect_statistics::mode(&v));
}