use std::{str::FromStr, env};

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Erro ao converter argumento para u64."));
    }
    
    if numbers.len() == 0 {
        eprintln!("Funcionamento: Calcula mcd na sequência fornecida ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = mdc(d, *m);
    }

    println!("O maior divisor comum de {:?} é {}", numbers, d);
}

fn mdc(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_mdc() {
    assert_eq!(mdc(14, 15), 1);
    assert_eq!(mdc(2*3*5*11*17, 
                   3*7*11*13*19
                   ), 3*11);
}