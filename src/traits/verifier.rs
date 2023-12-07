use bitcoin::Address;

pub trait VerifierTrait {
    fn create_challenge_tree(&self, circuit_size: usize) -> Address;
}
