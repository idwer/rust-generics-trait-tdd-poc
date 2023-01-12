use crate::poc::traits::ProofOfConcept;

pub struct EmptyProofOfConcept;

impl ProofOfConcept<String, f64> for EmptyProofOfConcept {
    fn poc(param: f64) -> (String, f64) {
        (String::from("TDD PoC for f64"), param * 2.0)
    }
}

impl ProofOfConcept<String, String> for EmptyProofOfConcept {
    fn poc(param: String) -> (String, String) {
        (String::from("TDD PoC for String"), param.clone())
    }
}

#[cfg(test)]
mod tests_solution {
    use super::*;

    #[test]
    fn test_poc_001() {
        assert_eq!(
            <EmptyProofOfConcept as ProofOfConcept<String, String>>::poc(String::from(
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
            <EmptyProofOfConcept as ProofOfConcept<String, f64>>::poc(1.5),
            (String::from("TDD PoC for f64"), 3.0)
        );
    }
}
