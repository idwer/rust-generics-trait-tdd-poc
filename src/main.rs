mod poc;

use crate::poc::proof_of_concept::EmptyProofOfConcept;
use crate::poc::traits::ProofOfConcept;

fn main() {
    println!(
        "{:?}",
        <EmptyProofOfConcept as ProofOfConcept<String, String>>::poc(String::from(
            "TDD PoC for String"
        ))
    );
    println!(
        "{:?}",
        <EmptyProofOfConcept as ProofOfConcept<String, f64>>::poc(1.5)
    );
}
