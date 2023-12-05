use std::str::FromStr;

use bitvmrs::utils::{bool_array_to_number, number_to_bool_array};
use bitvmrs::{circuit::Circuit, traits::circuit::CircuitTrait};

use bitcoin::blockdata::script::Builder;
use bitcoin::taproot::TaprootBuilder;
use bitcoin::opcodes::all::*;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use bitcoin::{ScriptBuf, XOnlyPublicKey};
use rand::Rng;

fn taptest() {
    let mut rng = rand::thread_rng();
    let preimage1: [u8; 32] = rng.gen();
    let preimage2: [u8; 32] = rng.gen();
    let hash1 = sha256::Hash::hash(&preimage1).to_byte_array();
    let hash2 = sha256::Hash::hash(&preimage2).to_byte_array();
    let script1 = Builder::new()
        .push_opcode(OP_SHA256)
        .push_slice(hash1)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_SHA256)
        .push_slice(hash2)
        .push_opcode(OP_EQUAL)
        .into_script();
    let script2 = Builder::new()
        .push_opcode(OP_SHA256)
        .push_slice(hash1)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_SHA256)
        .push_slice(hash2)
        .push_opcode(OP_EQUAL)
        .into_script();
    let script3 = Builder::new()
        .push_opcode(OP_SHA256)
        .push_slice(hash1)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_SHA256)
        .push_slice(hash2)
        .push_opcode(OP_EQUAL)
        .into_script();
    let script4 = Builder::new()
        .push_opcode(OP_SHA256)
        .push_slice(hash1)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_SHA256)
        .push_slice(hash2)
        .push_opcode(OP_EQUAL)
        .into_script();
    let mut taproot = TaprootBuilder::new();
    taproot = taproot.add_leaf(2, script1).unwrap();
    taproot = taproot.add_leaf(2, script2).unwrap();
    taproot = taproot.add_leaf(1, script3).unwrap();
    //taproot = taproot.add_leaf(2, script4).unwrap();
    let taptree = taproot.try_into_taptree().unwrap();
    println!("{:?}", taptree.node_info());
}

fn main() {
    println!("Hello, world!");
    //taptest();
    let circuit = Circuit::from_bristol("bristol/test.txt");
    let pk = XOnlyPublicKey::from_str("93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51").unwrap();
    circuit.generate_anti_contradiction_tree(pk);
}
