use bitcoin::Address;

pub trait VerifierTrait {
    fn create_challenge_tree(&self, circuit_size: usize) -> Address;
    fn create_challenge_hashes(&self, circuit_size: usize) -> Vec<[u8; 32]>;
}
