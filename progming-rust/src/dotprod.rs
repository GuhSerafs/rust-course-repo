use std::ops::{Add, Mul};

pub fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Mul<Output = N> + Add<Output = N> + Default + Copy,
{
    let mut sum: N = N::default();
    for i in 0..v1.len() {
        sum = sum + v1[i] * v2[i];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::dot;

    #[test]
    fn dot_integers() {
        let vect1 = [1, 2, 3, 4];
        let vect2 = [2, 4, 6, 8];
        assert_eq!(dot(&vect1, &vect2), 60);
    }

    #[test]
    fn dot_floats() {
        let vect1 = [1.0, 2.0, 3.0, 4.0];
        let vect2 = [2.0, 4.0, 6.0, 8.0];
        assert_eq!(dot(&vect1, &vect2), 60.0);
    }
}
