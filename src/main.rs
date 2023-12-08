use bitcoin::absolute::{Height, LockTime};
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::consensus::Decodable;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::sighash::SighashCache;
use bitcoin::taproot::LeafVersion;
use bitcoin::{Amount, OutPoint, ScriptBuf, TapLeafHash, Transaction, TxIn, TxOut, Witness};

use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitvm::actor::Actor;
use bitvm::traits::wire::WireTrait;

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
    // if rpc feature is enabled, use the following code to connect to a bitcoin node
    let rpc = Client::new(
        "http://localhost:18443/wallet/admin",
        Auth::UserPass("admin".to_string(), "admin".to_string()),
    )
    .unwrap_or_else(|e| panic!("Failed to connect to Bitcoin RPC: {}", e));

    let circuit = Circuit::from_bristol("bristol/add.txt");

    let paul = Actor::new();
    let mut vicky = Actor::new();
    let secp = Secp256k1::new();
    let amt = 10_000;

    let initial_fund = rpc
        .send_to_address(
            &paul.address,
            Amount::from_sat(amt),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap_or_else(|e| panic!("Failed to send to address: {}", e));
    let initial_tx = rpc
        .get_transaction(&initial_fund, None)
        .unwrap_or_else(|e| panic!("Failed to get transaction: {}", e));

    println!("initial tx = {:?}", initial_tx);

    // println!("Send {} satoshis to Public Key: {}", amt, paul.address);
    // let txid: Txid = take_stdin("Enter txid: ")
    //     .parse()
    //     .expect("invalid txid format");
    // let vout: u32 = take_stdin("Enter vout: ")
    //     .trim()
    //     .parse()
    //     .expect("invalid vout format");

    let challenge_hashes = vicky.generate_challenge_hashes(circuit.num_gates());

    let (address, kickoff_taproot_info) =
        circuit.generate_challenge_tree(&secp, &paul, &vicky, challenge_hashes);

    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::from(Height::MIN),
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: initial_fund,
                vout: initial_tx.details[0].vout,
            },
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
    let sig = paul.sign_with_tweak(sig_hash, None);
    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());

    println!("txid : {:?}", serialize_hex(&tx));

    let kickoff_tx = rpc
        .send_raw_transaction(&tx)
        .unwrap_or_else(|e| panic!("Failed to send raw transaction: {}", e));
    println!("initial kickoff tx = {:?}", kickoff_tx);

    // let mut txid_str: [u8];
    // tx.consensus_encode().unwrap();

    let wire_rcref = &circuit.wires[0];
    let wire = wire_rcref.try_borrow_mut().unwrap();

    let vout: u32 = 0;

    let script = wire.generate_anti_contradiction_script(vicky.public_key);

    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::from(Height::MIN),
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: kickoff_tx,
                vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            script_pubkey: vicky.address.script_pubkey(),
            value: Amount::from_sat(9000),
        }],
    };

    let mut sighash_cache = SighashCache::new(tx.borrow_mut());

    let prevouts = vec![TxOut {
        script_pubkey: address.script_pubkey(),
        value: Amount::from_sat(amt - 500),
    }];

    let sig_hash = sighash_cache
        .taproot_script_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&prevouts),
            TapLeafHash::from_script(&script, LeafVersion::TapScript),
            bitcoin::sighash::TapSighashType::Default,
        )
        .unwrap();

    let sig = vicky.sign(sig_hash);

    let control_block = kickoff_taproot_info
        .control_block(&(script.clone(), LeafVersion::TapScript))
        .expect("Cannot create control block");

    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());
    witness.push(wire.preimages.unwrap()[1]);
    witness.push(wire.preimages.unwrap()[0]);
    witness.push(script);
    witness.push(&control_block.serialize());

    println!("equivocation");
    println!("txid : {:?}", tx.txid());
    println!("txid : {:?}", serialize_hex(&tx));
    let eqv_tx = rpc
        .send_raw_transaction(&tx)
        .unwrap_or_else(|e| panic!("Failed to send raw transaction: {}", e));
    println!("eqv tx = {:?}", eqv_tx);
}
