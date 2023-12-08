use bitcoin::XOnlyPublicKey;

// prover.rs
use bitvm::{
    circuit::Circuit,
    communication::{receive_message, send_message},
    wire::HashTuple,
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

    let verifier_publickey_str: String = receive_message(&mut ws_stream).await.unwrap();
    println!("Verifier public key: {}", verifier_publickey_str);
    let verifier_publickey: XOnlyPublicKey = verifier_publickey_str.parse().unwrap();
    println!("Verifier public key: {}", verifier_publickey);

    let circuit = Circuit::from_bristol("bristol/add.txt");
    let wire_hashes: Vec<HashTuple> = circuit.get_wire_hashes();

    send_message(&mut ws_stream, &wire_hashes).await.unwrap();
}
