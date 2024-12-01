#[cfg(test)]
mod tests {
    use rust_25_days_challenge::day01::solution::solve_challenge;

    #[test]
    fn test_message_handling(){
        let message = String::from("Merry Christmas! Enjoy your gift!");
        let result = solve_challenge();
        assert!(result.contains(&message))
    }

    #[test]
    fn test_reference_handling() {
        let original = String::from("Original message");
        let reference = &original;
        assert_eq!(*reference,original)
    }
}