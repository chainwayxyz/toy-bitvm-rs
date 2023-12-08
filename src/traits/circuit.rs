use crate::actor::Actor;
use bitcoin::{
    secp256k1::{All, Secp256k1},
    taproot::TaprootSpendInfo,
    Address,
};

// This trait defines the behavior of a circuit.
pub trait CircuitTrait {
    fn num_gates(&self) -> usize;

    fn evaluate(&mut self, inputs: Vec<Vec<bool>>) -> Vec<Vec<bool>>;

    fn get_wire_hashes(&self) -> Vec<[[u8; 32]; 2]>;

    fn from_bristol(file: &str) -> Self;

    fn generate_challenge_tree(
        &self,
        secp: &Secp256k1<All>,
        prover: &Actor,
        verifier: &Actor,
        challenge_hashes: Vec<[u8; 32]>,
    ) -> (Address, TaprootSpendInfo);

    fn generate_response_tree(
        &self,
        secp: &Secp256k1<All>,
        prover: &Actor,
        verifier: &Actor,
        challenge_hashes: Vec<[u8; 32]>,
    ) -> (Address, TaprootSpendInfo);
}
