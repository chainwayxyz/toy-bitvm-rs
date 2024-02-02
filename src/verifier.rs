use std::borrow::BorrowMut;
use std::time::Duration;

use bitcoin::absolute::{Height, LockTime};
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::schnorr::Signature;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::SighashCache;
use bitcoin::taproot::LeafVersion;
use bitcoin::{secp256k1::Secp256k1, Transaction, Txid, XOnlyPublicKey};
use bitcoin::{Amount, OutPoint, ScriptBuf, TapLeafHash, TxIn, TxOut, Witness};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use toy_bitvm::{
    actor::Actor,
    circuit::wire::{HashTuple, HashValue, PreimageValue, Wire},
    circuit::Circuit,
    communication::{receive_message, send_message},
    transactions::{
        generate_2_of_2_script, generate_anti_contradiction_script,
        generate_challenge_address_and_info, generate_challenge_script,
        generate_equivoation_address_and_info, generate_response_address_and_info,
        generate_response_second_address_and_info, watch_transaction,
    },
    utils::take_stdin,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Listening on: 127.0.0.1:9000");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let circuit_bristol_path: String = receive_message(&mut ws_stream).await.unwrap();
    println!("Received: {}", circuit_bristol_path);

    let mut verifier = Actor::new();
    let verifier_public_key = verifier.public_key;
    println!("Verifier public key: {}", verifier_public_key);
    // send our public key to the prover
    send_message(&mut ws_stream, &verifier.public_key.to_string())
        .await
        .unwrap();
    let prover_public_key_str: String = receive_message(&mut ws_stream).await.unwrap();
    let prover_public_key: XOnlyPublicKey = prover_public_key_str.parse().unwrap();
    println!("Prover public key: {}", prover_public_key);

    // NOW PUBLIC KEY EXCHANGE IS COMPLETE

    let wire_hashes: Vec<HashTuple> = receive_message(&mut ws_stream).await.unwrap();

    let mut circuit = Circuit::from_bristol("bristol/add.txt", Some(wire_hashes));
    let secp = Secp256k1::new();

    let bisection_length = 10;

    let (equivocation_address, equivocation_taproot_info) = generate_equivoation_address_and_info(
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

    let initial_fund_txid: Txid = receive_message(&mut ws_stream).await.unwrap();
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
        println!("Bisection iteration: {}", i);
        let challenge_hashes: Vec<HashValue> =
            verifier.generate_challenge_hashes(circuit.num_gates());
        send_message(&mut ws_stream, &challenge_hashes)
            .await
            .unwrap();

        let (challenge_address, _) = generate_challenge_address_and_info(
            &secp,
            &circuit,
            prover_public_key,
            verifier_public_key,
            &challenge_hashes,
        );

        let (response_address, _) = generate_response_address_and_info(
            &secp,
            &circuit,
            prover_public_key,
            &challenge_hashes,
        );

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
            let sig = verifier.sign(sig_hash);
            send_message(&mut ws_stream, &sig).await.unwrap();
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
        let challenge_sig: Signature = receive_message(&mut ws_stream).await.unwrap();
        verifier.add_signature(challenge_sig);
        println!("Challenge Sig: {:?}", challenge_sig);
        // Verify needs to verify the signature
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
        secp.verify_schnorr(
            &challenge_sig,
            &Message::from_digest_slice(sig_hash.as_byte_array()).expect("should be hash"),
            &prover_public_key,
        )
        .unwrap();

        last_output = outputs2;
        last_txid = response_tx.txid();
    }
    println!("Bisection completed");
    let kickoff_txid: Txid = receive_message(&mut ws_stream).await.unwrap();
    if kickoff_tx.txid() != kickoff_txid {
        panic!("Kickoff txid mismatch!");
    }

    let mut challenge_gate_num: usize = 0;
    last_txid = initial_fund_txid;
    for i in 0..bisection_length as u64 {
        let challenge_hashes: Vec<HashValue> = verifier.get_challenge_hashes(i as usize);

        let (challenge_address, challenge_taproot_info) = generate_challenge_address_and_info(
            &secp,
            &circuit,
            prover_public_key,
            verifier_public_key,
            &challenge_hashes,
        );

        let (response_address, _) = generate_response_address_and_info(
            &secp,
            &circuit,
            prover_public_key,
            &challenge_hashes,
        );

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

        let challenge_tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::from(Height::MIN),
            input: inputs,
            output: outputs1.clone(),
        };

        let mut found_contradiction: Option<Wire> = None;

        if i != 0 {
            // Verifier needs needs to give signature to prover so that prover can give a response
            println!("Waiting for prover's response...");
            let provers_response =
                watch_transaction(&rpc, &challenge_tx.txid(), watch_interval).unwrap();
            let num_wires = circuit.gates[challenge_gate_num].get_input_size()
                + circuit.gates[challenge_gate_num].get_output_size();
            let mut witness: Vec<PreimageValue> = Vec::new();
            for i in 0..num_wires {
                witness.push(
                    provers_response.input[0]
                        .witness
                        .nth(i + 1)
                        .unwrap()
                        .try_into()
                        .expect("Slice with incorrect length"),
                );
            }

            found_contradiction =
                circuit.gates[challenge_gate_num].add_preimages_from_witness(witness);
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

        if let Some(value) = found_contradiction {
            let mut steal_tx = Transaction {
                version: bitcoin::transaction::Version::TWO,
                lock_time: LockTime::from(Height::MIN),
                input: vec![TxIn {
                    previous_output: OutPoint {
                        txid: challenge_tx.txid(),
                        vout: 1,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: bitcoin::transaction::Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                }],
                output: vec![TxOut {
                    script_pubkey: verifier.address.script_pubkey(),
                    value: Amount::from_sat(amt - (2 * i + 2) * (fee + dust_limit)),
                }],
            };
            println!("Contraditcion found! Slashing the prover!");
            let mut sighash_cache = SighashCache::new(steal_tx.borrow_mut());

            let equivocation_script = generate_anti_contradiction_script(
                value.clone().get_hash_pair(),
                verifier_public_key,
            );
            let sig_hash = sighash_cache
                .taproot_script_spend_signature_hash(
                    0,
                    &bitcoin::sighash::Prevouts::All(&[outputs1[1].clone()]),
                    TapLeafHash::from_script(&equivocation_script, LeafVersion::TapScript),
                    bitcoin::sighash::TapSighashType::Default,
                )
                .unwrap();

            let equivocation_sig = verifier.sign(sig_hash);

            let equivocation_control_block = equivocation_taproot_info
                .control_block(&(equivocation_script.clone(), LeafVersion::TapScript))
                .expect("Cannot create control block");

            let witness = sighash_cache.witness_mut(0).unwrap();
            witness.push(equivocation_sig.as_ref());
            witness.push(value.clone().preimages.unwrap().one.unwrap());
            witness.push(value.preimages.unwrap().zero.unwrap());
            witness.push(equivocation_script);
            witness.push(&equivocation_control_block.serialize());

            let steal_txid = rpc
                .send_raw_transaction(&steal_tx)
                .unwrap_or_else(|e| panic!("Failed to send raw transaction: {}", e));
            println!("Verifier slashed the prover: {:?}", steal_txid);
        }
        // Prover needs to give signature to verifier so that verifier can start a challenge
        challenge_gate_num =
            take_stdin("Enter your challenge gate if you want to challenge the prover\n").unwrap();
        let musig_presigned_by_prover = verifier.get_signature(i as usize);

        // println!("challenge sig: {:?}", challenge_sig);
        // Verify needs to verify the signature
        let mut sighash_cache = SighashCache::new(response_tx.borrow_mut());

        // Now we need to sign with the challenge_gate_num preimage to reveal a challenge
        let challenge_script = generate_challenge_script(
            prover_public_key,
            verifier_public_key,
            &challenge_hashes[challenge_gate_num],
        );
        let musig_2of2_script = generate_2_of_2_script(prover_public_key, verifier_public_key);

        let sig_hash = sighash_cache
            .taproot_script_spend_signature_hash(
                0,
                &bitcoin::sighash::Prevouts::All(&outputs1),
                TapLeafHash::from_script(&challenge_script, LeafVersion::TapScript),
                bitcoin::sighash::TapSighashType::Default,
            )
            .unwrap();
        let verifier_challenge_sig = verifier.sign(sig_hash);

        let sig_hash = sighash_cache
            .taproot_script_spend_signature_hash(
                1,
                &bitcoin::sighash::Prevouts::All(&outputs1),
                TapLeafHash::from_script(&musig_2of2_script, LeafVersion::TapScript),
                bitcoin::sighash::TapSighashType::Default,
            )
            .unwrap();
        let verifier_2of2_sig = verifier.sign(sig_hash);
        let challenge_preimage = verifier.get_challenge_preimage(i as usize, challenge_gate_num);

        let challenge_control_block = challenge_taproot_info
            .control_block(&(challenge_script.clone(), LeafVersion::TapScript))
            .expect("Cannot create control block");

        let musig_control_block = equivocation_taproot_info
            .control_block(&(musig_2of2_script.clone(), LeafVersion::TapScript))
            .expect("Cannot create control block");

        let witness0 = sighash_cache.witness_mut(0).unwrap();
        witness0.push(verifier_challenge_sig.as_ref());
        witness0.push(challenge_preimage);
        witness0.push(challenge_script);
        witness0.push(&challenge_control_block.serialize());

        let witness1 = sighash_cache.witness_mut(1).unwrap();
        witness1.push(verifier_2of2_sig.as_ref());
        witness1.push(musig_presigned_by_prover.as_ref());
        witness1.push(musig_2of2_script);
        witness1.push(&musig_control_block.serialize());

        let response_txid = rpc
            .send_raw_transaction(&response_tx)
            .unwrap_or_else(|e| panic!("Failed to send raw transaction: {}", e));

        println!("Challenge transaction sent! txid: {:?}", response_txid);

        last_output = outputs2;
        last_txid = response_tx.txid();
    }
    println!("Last output: {:?}", last_output);
}
