#[cfg(test)]
mod test {
    use ::libhermit_rs_bench::fibonacci;

    #[test]
    fn test_fibonacci_loop() {
        let expected_results = vec![
            1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        ];
        for i in 0..20 {
            assert_eq!(fibonacci::as_loop(i), expected_results[i as usize])
        }
    }

    #[test]
    fn test_fibonacci_recursive() {
        let expected_results = vec![
            1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        ];
        for i in 0..20 {
            assert_eq!(fibonacci::as_recursive(i), expected_results[i as usize])
        }
    }
}
