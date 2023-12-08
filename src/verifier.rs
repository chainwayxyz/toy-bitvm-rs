// verifier.rs
use bitvm::{
    actor::Actor,
    communication::{receive_message, send_message},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

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

    let message: String = receive_message(&mut ws_stream).await.unwrap();
    println!("Received: {}", message);

    let verifier = Actor::new();

    // send our public key to the prover
    send_message(&mut ws_stream, &verifier.public_key.to_string())
        .await
        .unwrap();

    let wire_hashes: Vec<[[u8; 32]; 2]> = receive_message(&mut ws_stream).await.unwrap();

    println!("Wire hashes: {:?}", wire_hashes);
}
