pub trait ProofOfConcept<String, T> {
    fn poc(message: T) -> (String, T);
}
