#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod vect_statistics {
    pub fn average(v: Vec<isize>) -> f64 {
        let sum: isize = v.iter().sum();
        let length: f64 = v.len() as f64;
        return sum as f64/length;
    }
    fn mode() {}
    fn mean() {}
}