#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod vect_statistics {
    use std::collections::HashMap;

    enum EvenOrOdd {
        Even, 
        Odd
    }

    impl From<isize> for EvenOrOdd {
        fn from(x: isize) -> EvenOrOdd {
            if x % 2 == 0 {
                return EvenOrOdd::Even;
            } else {
                return EvenOrOdd::Odd;
            }
        }
    }

    pub fn average(v: &Vec<isize>) -> f64 {
        let sum: isize = v.iter().sum();
        let length: f64 = v.len() as f64;
        return sum as f64/length;
    }
    
    pub fn median(v: &Vec<isize>) -> f64 {
        let mut sorted = v.clone();
        sorted.sort();
        let length_type = EvenOrOdd::from(v.len() as isize);
        let half = v.len() / 2;
        
        match length_type {
            EvenOrOdd::Even => {
                return (sorted[half] + sorted[half-1]) as f64/2.0;
            }
            EvenOrOdd::Odd => {
                return sorted[half] as f64;
            }
        }
    }

    pub fn mode(v: &Vec<isize>) -> Vec<isize> {
        let mut incidences = HashMap::new();
        let mut maxv: isize = 1;
        let mut modes: Vec<isize> = Vec::new();
        for item in v.iter() {
            let count = incidences.entry(item).or_insert(0); // Retorna uma referência ao valor
            *count += 1;
        }
        for (_k, v) in incidences.iter() {
            if v > &maxv {
                maxv = *v;
            }
        }
        if (maxv) != 1 {
            for (k, v) in incidences.iter() {
                if v == &maxv {
                    modes.push(**k);
                }
            }
        }
        return modes;
    }
}