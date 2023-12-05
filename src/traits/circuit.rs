//use bitcoin::taproot::TapTree;

use bitcoin::XOnlyPublicKey;

// This trait defines the behavior of a circuit.
pub trait CircuitTrait {
    fn evaluate(&mut self, inputs: Vec<Vec<bool>>) -> Vec<Vec<bool>>;

    fn from_bristol(file: &str) -> Self;

    fn generate_bit_commitment_tree(&self);

    fn generate_anti_contradiction_tree(&self, prover_pk: XOnlyPublicKey);
}
