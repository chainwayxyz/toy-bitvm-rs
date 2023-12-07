use bitcoin::absolute::{Height, LockTime};
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::consensus::Decodable;
use bitcoin::hash_types::Txid;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::sighash::SighashCache;
use bitcoin::{Amount, OutPoint, ScriptBuf, Transaction, TxIn, TxOut, Witness};

use bitvm::actor::Actor;
use bitvm::utils::take_cmd_input;
use bitvm::{circuit::Circuit, traits::circuit::CircuitTrait};

use std::borrow::BorrowMut;

pub fn parse_hex_transaction(
    tx_hex: &str,
) -> Result<Transaction, bitcoin::consensus::encode::Error> {
    if let Ok(reader) = hex::decode(tx_hex) {
        Transaction::consensus_decode(&mut &reader[..])
    } else {
        Err(bitcoin::consensus::encode::Error::ParseFailed(
            "Could not decode hex",
        ))
    }
}

fn main() {
    let circuit = Circuit::from_bristol("bristol/add.txt");

    let paul = Actor::new();
    let vicky = Actor::new();
    let secp = Secp256k1::new();
    let amt = 10_000;

    println!("Send {} satoshis to Public Key: {}", amt, paul.address);

    let txid: Txid = take_cmd_input("Enter txid: ").parse().expect("invalid txid format");
    let vout: u32 = take_cmd_input("Enter vout: ").trim().parse().expect("invalid vout format");

    let (address, _info) = circuit
    .generate_anti_contradiction_tree(&secp, &paul, &vicky);

    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::from(Height::MIN),
        input: vec![TxIn {
            previous_output: OutPoint { txid, vout },
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            script_pubkey: address.script_pubkey(),
            value: Amount::from_sat(amt - 500),
        }],
    };

    let prevouts = vec![TxOut {
        script_pubkey: paul.address.script_pubkey(),
        value: Amount::from_sat(amt),
    }];

    println!("prevout: {:?}", prevouts);
    let mut sighash_cache = SighashCache::new(tx.borrow_mut());
    // TODO: add support for signing with a keypair
    let sig_hash = sighash_cache
        .taproot_key_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&prevouts),
            bitcoin::sighash::TapSighashType::Default,
        )
        .unwrap();

    // Witness::from_slice(sigHash)
    let sig = paul.sign(sig_hash);
    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());

    println!("sigHash : {:?}", sig_hash);
    println!("tx : {:?}", tx);
    println!("txid : {:?}", tx.txid());
    println!("txid : {:?}", serialize_hex(&tx));
    // let mut txid_str: [u8];
    // tx.consensus_encode().unwrap();
}
