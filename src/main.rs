use bitcoin::absolute::{Height, LockTime};

use bitcoin::consensus::encode::serialize_hex;
use bitcoin::hash_types::Txid;
use bitcoin::sighash::SighashCache;
use bitcoin::{Amount, OutPoint, ScriptBuf, Transaction, TxIn, TxOut, Witness};
use bitvmrs::prover::Prover;
use bitvmrs::utils::{bool_array_to_number, number_to_bool_array};
use bitvmrs::verifier::Verifier;
use bitvmrs::{circuit::Circuit, traits::circuit::CircuitTrait};

use std::io::{self, Write}; // Import necessary modules

fn main() {
    // println!("Hello, world!");
    let mut circuit = Circuit::from_bristol("bristol/add.txt");
    let a1 = 633;
    let a2 = 15;
    let b1 = number_to_bool_array(a1, 64);
    let b2 = number_to_bool_array(a2, 64);

    let o = circuit.evaluate(vec![b1, b2]);
    let output = bool_array_to_number(o.get(0).unwrap().to_vec());
    println!("output : {:?}", output);
    assert_eq!(output, a1 + a2);

    let paul = Prover::new();
    let vicky = Verifier::new();
    let amt = 10_000;

    println!("Send {} satoshis to Public Key: {}", amt, paul.address);

    let mut txid_str = String::new();
    let mut vout_str = String::new();

    print!("Enter txid: ");
    io::stdout().flush().unwrap(); // Make sure 'Enter txid' is printed before input
    io::stdin()
        .read_line(&mut txid_str)
        .expect("Failed to read txid");
    let txid_str = txid_str.trim(); // Trim newline/whitespace
    let txid: Txid = txid_str.parse().expect("Invalid txid format");

    // Read vout
    print!("Enter vout: ");
    io::stdout().flush().unwrap(); // Make sure 'Enter vout' is printed before input
    io::stdin()
        .read_line(&mut vout_str)
        .expect("Failed to read vout");
    let vout: u32 = vout_str.trim().parse().expect("Invalid vout format");

    // let txid: Txid = "9aa3e28ba1742b0df567df6998c00ef78136be16dd107f422f8af9b0f56bd68c".parse().unwrap();
    // let vout: u32 = "0".parse().unwrap();

    let mut tx = Transaction {
        version: bitcoin::transaction::Version::ONE,
        lock_time: LockTime::from(Height::MIN),
        input: vec![TxIn {
            previous_output: OutPoint { txid, vout },
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::transaction::Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            script_pubkey: circuit
                .generate_anti_contradiction_tree(paul.public_key, vicky.public_key)
                .script_pubkey(),
            value: Amount::from_sat(amt - 500),
        }],
    };
    let prevouts = vec![TxOut {
        script_pubkey: paul.address.script_pubkey(),
        value: Amount::from_sat(amt),
    }];
    // TODO: add support for signing with a keypair
    let sig_hash = SighashCache::new(tx.clone())
        .taproot_key_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&prevouts),
            bitcoin::sighash::TapSighashType::Default,
        )
        .unwrap();

    // Witness::from_slice(sigHash)
    let sig = paul.sign(sig_hash);

    let x = sig.as_ref();

    let witness = Witness::from_slice(&[x]);

    tx.input[0].witness = witness;
    println!("sigHash : {:?}", sig_hash);
    println!("tx : {:?}", tx);
    println!("txid : {:?}", tx.txid());
    println!("txid : {:?}", serialize_hex(&tx));
    // let mut txid_str: [u8];
    // tx.consensus_encode().unwrap();
}
