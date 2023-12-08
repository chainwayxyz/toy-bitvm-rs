use std::str::FromStr;

use bitcoin::secp256k1::{All, Secp256k1};
use bitcoin::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::{Address, ScriptBuf, XOnlyPublicKey};


use bitcoin::blockdata::script::Builder;
use bitcoin::opcodes::all::*;

use crate::wire::{HashValue, HashTuple};

use crate::circuit::Circuit;

pub fn taproot_address_from_script_leaves(
    secp: &Secp256k1<All>,
    scripts: Vec<ScriptBuf>,
) -> (Address, TaprootSpendInfo) {
    let n = scripts.len();
    assert!(n > 1, "more than one script is required");
    let m: u8 = ((n - 1).ilog2() + 1) as u8; // m = ceil(log(n))
    let k = 2_usize.pow(m.into()) - n;
    let taproot = (0..n).fold(TaprootBuilder::new(), |acc, i| {
        acc.add_leaf(m - ((i >= n - k) as u8), scripts[i].clone())
            .unwrap()
    });
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )
    .unwrap();
    let tree_info = taproot.finalize(secp, internal_key).unwrap();
    let address = Address::p2tr(
        secp,
        internal_key,
        tree_info.merkle_root(),
        bitcoin::Network::Regtest,
    );
    (address, tree_info)
}

pub fn generate_response_address_and_info(
    secp: &Secp256k1<All>,
    circuit: &Circuit,
    verifier_pk: XOnlyPublicKey,
    challenge_hashes: Vec<HashValue>,
) -> (Address, TaprootSpendInfo) {
    assert_eq!(
        challenge_hashes.len(),
        circuit.gates.len(),
        "wrong number of challenge hashes"
    );
    let mut scripts = circuit
        .gates
        .iter()
        .zip(challenge_hashes.iter())
        .map(|(gate, hash)| gate.create_response_script(*hash))
        .collect::<Vec<ScriptBuf>>();
    scripts.push(generate_timelock_script(verifier_pk, 10));
    taproot_address_from_script_leaves(secp, scripts)
}

pub fn generate_challenge_address_and_info(
    secp: &Secp256k1<All>,
    circuit: &Circuit,
    prover_pk: XOnlyPublicKey,
    verifier_pk: XOnlyPublicKey,
    challenge_hashes: Vec<HashValue>,
) -> (Address, TaprootSpendInfo) {
    assert_eq!(
        challenge_hashes.len(),
        circuit.gates.len(),
        "wrong number of challenge hashes"
    );
    let mut scripts = challenge_hashes
        .iter()
        .map(|x| generate_challenge_script(prover_pk, verifier_pk, x))
        .collect::<Vec<ScriptBuf>>();
    // let mut reveal_challenge_scripts =
    scripts.extend(circuit.wires.iter().map(|wire_rcref| {
        generate_anti_contradiction_script(wire_rcref.try_borrow_mut().unwrap().get_hash_pair(), verifier_pk)
    }));
    scripts.push(generate_timelock_script(prover_pk, 10));
    taproot_address_from_script_leaves(secp, scripts)
}

pub fn generate_anti_contradiction_script(wire_bit_hashes: HashTuple, verifier_pk: XOnlyPublicKey) -> ScriptBuf {
    Builder::new()
        .push_opcode(OP_SHA256)
        .push_slice(wire_bit_hashes.zero)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_SHA256)
        .push_slice(wire_bit_hashes.one)
        .push_opcode(OP_EQUALVERIFY)
        .push_x_only_key(&verifier_pk)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}

pub fn add_bit_commitment_script(wire_bit_hashes: HashTuple, builder: Builder) -> Builder {
    builder
        .push_opcode(OP_SHA256)
        .push_opcode(OP_DUP)
        .push_slice(wire_bit_hashes.one)
        .push_opcode(OP_EQUAL)
        .push_opcode(OP_DUP)
        .push_opcode(OP_ROT)
        .push_slice(wire_bit_hashes.zero)
        .push_opcode(OP_EQUAL)
        .push_opcode(OP_BOOLOR)
        .push_opcode(OP_VERIFY)
}

pub fn generate_challenge_script(prover_pk: XOnlyPublicKey, verifier_pk: XOnlyPublicKey, challenge_hash: &HashValue) -> ScriptBuf {
    Builder::new()
        .push_slice(challenge_hash)
        .push_opcode(OP_EQUALVERIFY)
        .push_x_only_key(&prover_pk)
        .push_opcode(OP_CHECKSIGVERIFY)
        .push_x_only_key(&verifier_pk)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}

pub fn generate_timelock_script(actor_pk: XOnlyPublicKey, block_count: u32) -> ScriptBuf {
    Builder::new()
        .push_int(block_count as i64)
        .push_opcode(OP_CSV)
        .push_x_only_key(&actor_pk)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}
