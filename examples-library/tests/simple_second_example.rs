#[cfg(test)]
mod tests {
    use examples_library::SimpleCircle;

    #[test]
    fn check_distance_from_origin() {
        let circle = SimpleCircle::new((3.0, 4.0), 5.0);
        assert_eq!(circle.distance_from_origin(), 5.0);
    }
}
