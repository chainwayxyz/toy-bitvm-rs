use std::borrow::BorrowMut;
use std::time::Duration;

use bitcoin::absolute::{Height, LockTime};

use bitcoin::hashes::{sha256, Hash};
use bitcoin::secp256k1::schnorr::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::SighashCache;
use bitcoin::taproot::LeafVersion;
use bitcoin::{secp256k1::Secp256k1, Amount, Transaction, XOnlyPublicKey};
use bitcoin::{OutPoint, ScriptBuf, TapLeafHash, TxIn, TxOut, Witness};

use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitvm::transactions::{
    generate_2_of_2_script, generate_equivoation_address_and_info,
    generate_response_second_address_and_info, watch_transaction,
};

use bitvm::wire::PreimageValue;
// prover.rs
use bitvm::{
    actor::Actor,
    circuit::Circuit,
    communication::{receive_message, send_message},
    transactions::{generate_challenge_address_and_info, generate_response_address_and_info},
    wire::{HashTuple, HashValue},
};

use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:9000";
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    send_message(&mut ws_stream, &"bristol/add.txt".to_string())
        .await
        .unwrap();

    let verifier_public_key_str: String = receive_message(&mut ws_stream).await.unwrap();
    let verifier_public_key: XOnlyPublicKey = verifier_public_key_str.parse().unwrap();
    println!("Verifier public key: {}", verifier_public_key);
    let mut prover = Actor::new();
    let prover_public_key = prover.public_key;
    println!("Prover public key: {}", prover_public_key);
    send_message(&mut ws_stream, &prover_public_key.to_string())
        .await
        .unwrap();

    // NOW PUBLIC KEY EXCHANGE IS COMPLETE

    let circuit = Circuit::from_bristol("bristol/add.txt", None);
    let secp = Secp256k1::new();
    let wire_hashes: Vec<HashTuple> = circuit.get_wire_hashes();

    send_message(&mut ws_stream, &wire_hashes).await.unwrap();

    let bisection_length = 10;

    let (equivocation_address, _) = generate_equivoation_address_and_info(
        &secp,
        &circuit,
        prover_public_key,
        verifier_public_key,
    );

    let (response_second_address, _) =
        generate_response_second_address_and_info(&secp, prover_public_key, verifier_public_key);

    let rpc = Client::new(
        "http://localhost:18443/wallet/admin",
        Auth::UserPass("admin".to_string(), "admin".to_string()),
    )
    .unwrap_or_else(|e| panic!("Failed to connect to Bitcoin RPC: {}", e));

    let amt: u64 = 100_000;
    let fee: u64 = 500;
    let dust_limit: u64 = 546;
    let watch_interval = Duration::from_secs(1);

    let initial_fund_txid = rpc
        .send_to_address(
            &prover.address,
            Amount::from_sat(amt),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap_or_else(|e| panic!("Failed to send to address: {}", e));

    // Send the initial fund address to the verifier
    send_message(&mut ws_stream, &initial_fund_txid)
        .await
        .unwrap();

    let initial_fund_tx = rpc
        .get_transaction(&initial_fund_txid, None)
        .unwrap_or_else(|e| panic!("Failed to get transaction: {}", e));

    let mut last_txid = initial_fund_txid;
    let _last_vout = initial_fund_tx.details[0].vout;
    let mut last_output: Vec<TxOut> = Vec::new();
    let mut kickoff_tx: Transaction = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::from(Height::MIN),
        input: vec![],
        output: vec![],
    };

    for i in 0..bisection_length as u64 {
        println!("Bisection iteration {}", i);
        let challenge_hashes: Vec<HashValue> = receive_message(&mut ws_stream).await.unwrap();
        prover.add_challenge_hashes(challenge_hashes.clone());
        let (challenge_address, _) = generate_challenge_address_and_info(
            &secp,
            &circuit,
            prover_public_key,
            verifier_public_key,
            &challenge_hashes,
        );

        let (response_address, _) =
            generate_response_address_and_info(&secp, &circuit, &challenge_hashes);

        let outputs1 = vec![
            TxOut {
                script_pubkey: challenge_address.script_pubkey(),
                value: Amount::from_sat(dust_limit),
            },
            TxOut {
                script_pubkey: equivocation_address.script_pubkey(),
                value: Amount::from_sat(amt - (2 * i + 1) * (fee + dust_limit)),
            },
        ];

        let outputs2 = vec![
            TxOut {
                script_pubkey: response_address.script_pubkey(),
                value: Amount::from_sat(dust_limit),
            },
            TxOut {
                script_pubkey: response_second_address.script_pubkey(),
                value: Amount::from_sat(amt - (2 * i + 2) * (fee + dust_limit)),
            },
        ];

        let inputs = if i == 0 {
            vec![TxIn {
                previous_output: OutPoint {
                    txid: initial_fund_txid,
                    vout: initial_fund_tx.details[0].vout,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::new(),
            }]
        } else {
            vec![
                TxIn {
                    previous_output: OutPoint {
                        txid: last_txid,
                        vout: 0,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
                TxIn {
                    previous_output: OutPoint {
                        txid: last_txid,
                        vout: 1,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
            ]
        };

        let mut challenge_tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::from(Height::MIN),
            input: inputs,
            output: outputs1.clone(),
        };

        if i != 0 {
            // Verifier needs needs to give signature to prover so that prover can give a response
            let mut sighash_cache = SighashCache::new(challenge_tx.borrow_mut());

            let sig_hash = sighash_cache
                .taproot_script_spend_signature_hash(
                    1_usize,
                    &bitcoin::sighash::Prevouts::All(&last_output),
                    TapLeafHash::from_script(
                        &generate_2_of_2_script(prover_public_key, verifier_public_key),
                        LeafVersion::TapScript,
                    ),
                    bitcoin::sighash::TapSighashType::Default,
                )
                .unwrap();
            let response_sig: Signature = receive_message(&mut ws_stream).await.unwrap();
            secp.verify_schnorr(
                &response_sig,
                &Message::from_digest_slice(sig_hash.as_byte_array()).expect("should be hash"),
                &verifier_public_key,
            )
            .unwrap();
        } else {
            kickoff_tx = challenge_tx.clone();
        }

        let mut response_tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::from(Height::MIN),
            input: vec![
                TxIn {
                    previous_output: OutPoint {
                        txid: challenge_tx.txid(),
                        vout: 0,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
                TxIn {
                    previous_output: OutPoint {
                        txid: challenge_tx.txid(),
                        vout: 1,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
            ],
            output: outputs2.clone(),
        };
        // Prover needs to give signature to verifier so that verifier can start a challenge
        let mut sighash_cache = SighashCache::new(response_tx.borrow_mut());

        let sig_hash = sighash_cache
            .taproot_script_spend_signature_hash(
                1_usize,
                &bitcoin::sighash::Prevouts::All(&outputs1),
                TapLeafHash::from_script(
                    &generate_2_of_2_script(prover_public_key, verifier_public_key),
                    LeafVersion::TapScript,
                ),
                bitcoin::sighash::TapSighashType::Default,
            )
            .unwrap();
        let challenge_sig = prover.sign(sig_hash);
        println!("challenge sig: {:?}", challenge_sig);

        send_message(&mut ws_stream, &challenge_sig).await.unwrap();

        last_output = outputs2;
        last_txid = response_tx.txid();
    }
    println!("Bisection complete");
    // now we send the funding

    let prevouts = vec![TxOut {
        script_pubkey: prover.address.script_pubkey(),
        value: Amount::from_sat(amt),
    }];

    // if kickoff_tx uninitialized, then panic

    println!("prevout: {:?}", prevouts);
    let mut sighash_cache = SighashCache::new(kickoff_tx.borrow_mut());
    // TODO: add support for signing with a keypair
    let sig_hash = sighash_cache
        .taproot_key_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&prevouts),
            bitcoin::sighash::TapSighashType::Default,
        )
        .unwrap();

    // Witness::from_slice(sigHash)
    let sig = prover.sign_with_tweak(sig_hash, None);
    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());

    // println!("txid : {:?}", serialize_hex(&tx));

    let kickoff_txid = rpc
        .send_raw_transaction(&kickoff_tx)
        .unwrap_or_else(|e| panic!("Failed to send raw transaction: {}", e));
    println!("initial kickoff txid = {:?}", kickoff_txid);
    send_message(&mut ws_stream, &kickoff_txid).await.unwrap();

    let mut challenge_preimage: PreimageValue = [0; 32];
    let mut challenge_gate_index: usize = 0;
    last_txid = initial_fund_txid;
    for i in 0..bisection_length as u64 {
        let challenge_hashes: Vec<HashValue> = prover.get_challenge_hashes(i as usize);

        let (challenge_address, _challenge_taproot_info) = generate_challenge_address_and_info(
            &secp,
            &circuit,
            prover_public_key,
            verifier_public_key,
            &challenge_hashes,
        );

        let (response_address, _) =
            generate_response_address_and_info(&secp, &circuit, &challenge_hashes);

        let outputs1 = vec![
            TxOut {
                script_pubkey: challenge_address.script_pubkey(),
                value: Amount::from_sat(dust_limit),
            },
            TxOut {
                script_pubkey: equivocation_address.script_pubkey(),
                value: Amount::from_sat(amt - (2 * i + 1) * (fee + dust_limit)),
            },
        ];

        let outputs2 = vec![
            TxOut {
                script_pubkey: response_address.script_pubkey(),
                value: Amount::from_sat(dust_limit),
            },
            TxOut {
                script_pubkey: response_second_address.script_pubkey(),
                value: Amount::from_sat(amt - (2 * i + 2) * (fee + dust_limit)),
            },
        ];

        let inputs = if i == 0 {
            vec![TxIn {
                previous_output: OutPoint {
                    txid: initial_fund_txid,
                    vout: initial_fund_tx.details[0].vout,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::new(),
            }]
        } else {
            vec![
                TxIn {
                    previous_output: OutPoint {
                        txid: last_txid,
                        vout: 0,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
                TxIn {
                    previous_output: OutPoint {
                        txid: last_txid,
                        vout: 1,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
            ]
        };

        let mut challenge_tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::from(Height::MIN),
            input: inputs,
            output: outputs1.clone(),
        };

        if i != 0 {
            // Verifier needs needs to give signature to prover so that prover can give a response
            let mut sighash_cache = SighashCache::new(challenge_tx.borrow_mut());

            let _sig_hash = sighash_cache
                .taproot_script_spend_signature_hash(
                    1_usize,
                    &bitcoin::sighash::Prevouts::All(&last_output),
                    TapLeafHash::from_script(
                        &generate_2_of_2_script(prover_public_key, verifier_public_key),
                        LeafVersion::TapScript,
                    ),
                    bitcoin::sighash::TapSighashType::Default,
                )
                .unwrap();
            // let _sig = verifier.sign(sig_hash);
            println!("NOW WE GIVE THE RESPONSEEE");
            println!(
                "Challenge gate and preimage is: {:?}, {:?}",
                challenge_gate_index, challenge_preimage
            );
            return;
            // send_message(&mut ws_stream, &sig).await.unwrap();
        }

        let response_tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::from(Height::MIN),
            input: vec![
                TxIn {
                    previous_output: OutPoint {
                        txid: challenge_tx.txid(),
                        vout: 0,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
                TxIn {
                    previous_output: OutPoint {
                        txid: challenge_tx.txid(),
                        vout: 1,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                },
            ],
            output: outputs2.clone(),
        };

        // println!("response txid: {:?}", response_tx.txid());
        // Prover waits for challenge
        let challenge_tx = watch_transaction(&rpc, &response_tx.txid(), watch_interval).unwrap();
        let preimage: &[u8; 32] = challenge_tx.input[0]
            .witness
            .nth(1)
            .unwrap()
            .try_into()
            .expect("Slice with incorrect length");
        challenge_preimage = preimage.to_owned();
        let challenge_hash = sha256::Hash::hash(&challenge_preimage).to_byte_array();
        // println!("Challenged preimage: {:?}", challenge_preimage);
        // println!("Challenged hash: {:?}", challenge_hash);
        // find the challenge hash in the challenge hashes
        let mut challenge_index = 0;
        for (i, hash) in challenge_hashes.iter().enumerate() {
            if hash == &challenge_hash {
                challenge_index = i;
                break;
            }
        }
        challenge_gate_index = challenge_index;

        last_output = outputs2;
        last_txid = response_tx.txid();
    }
}
