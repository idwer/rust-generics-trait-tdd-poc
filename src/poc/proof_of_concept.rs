#[cfg(test)]
mod tests_solution {
    use super::*;

    #[test]
    fn test_poc_001() {
        assert_eq!(
            poc_string(String::from(
                "TDD PoC for String"
            )),
            (
                String::from("TDD PoC for String"),
                String::from("TDD PoC for String")
            )
        );
    }

    #[test]
    fn test_poc_002() {
        assert_eq!(
            poc_f64(1.5),
            (String::from("TDD PoC for f64"), 3.0)
        );
    }
}
