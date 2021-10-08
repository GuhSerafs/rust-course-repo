#[cfg(test)]
mod tests {
    use examples_library::SimpleCircle;

    #[test]
    fn check_cicle_area() {
        let circle = SimpleCircle::new((0.0, 0.0), 5.0);
        assert_eq!(circle.area(), 3.14159*5.0*5.0);
    }
}
