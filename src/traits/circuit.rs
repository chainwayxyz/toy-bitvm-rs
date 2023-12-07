use crate::actor::Actor;
use bitcoin::{
    secp256k1::{All, Secp256k1},
    taproot::TaprootSpendInfo,
    Address,
};

// This trait defines the behavior of a circuit.
pub trait CircuitTrait {
    fn evaluate(&mut self, inputs: Vec<Vec<bool>>) -> Vec<Vec<bool>>;

    fn from_bristol(file: &str) -> Self;

    fn generate_bit_commitment_tree(&self);

    fn generate_anti_contradiction_tree(
        &self,
        secp: &Secp256k1<All>,
        prover: &Actor,
        verifier: &Actor,
    ) -> (Address, TaprootSpendInfo);
}
