pub trait ProofOfConcept<String, T> {
    fn poc(param: T) -> (String, T);
}
