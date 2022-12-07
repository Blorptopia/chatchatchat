use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;
use chatchatchat::models;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").await.expect("Failed to bind");
    println!("Listening for connections");
    
    let connected_sockets: Arc<RwLock<Vec<mpsc::UnboundedSender<Vec<u8>>>>> = Arc::new(RwLock::new(Vec::new()));

    // Accept connections
    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        println!("Connected: {addr}");

        tokio::spawn(handler(socket, connected_sockets.clone()));
    }
}

async fn handler(socket: TcpStream, connected_sockets: Arc<RwLock<Vec<mpsc::UnboundedSender<Vec<u8>>>>>) -> Result<(), std::io::Error>{
    let mut socket = socket;
    let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

    {
        connected_sockets.write().await.push(tx);
    }
    loop {
        // Get which one gets a result first.
        // Either someone dispatches a event or our client sends a message
        tokio::select! {
            size = socket.read_u32() => {
                // We need the size of the data to read only that many bytes
                let size = size.unwrap();
                println!("Got {size} size");
                
                // Create a buffer and change the size to what the client provided.
                let mut buf = Vec::new();
                buf.resize(size as usize, 0u8);

                // Read what they sent into the buffer
                socket.read_exact(&mut buf).await.expect("Failed to read");
                
                // Convert it to a string (it was bytes)
                let text = String::from_utf8(buf.clone()).expect("Failed to convert text");
                
                // Convert it to a NetworkPayload
                let _value: models::NetworkPayload = serde_json::from_str(&text).expect("Failed to convert");
                
                // Get everyone connected
                let receivers = connected_sockets.read().await;
                
                // Send the message to them so they can send it to the socket
                for receiver in receivers.iter() {
                    receiver.send(buf.clone()).expect("Failed to send");
                }
            }
            message = rx.recv() => {
                // Received broadcast, sending message to the socket
                let message = message.unwrap();
                println!("Received broadcast");
                
                // Size needs to be first
                socket.write_u32(message.len() as u32).await.expect("Failed to send size");
                socket.write(&message).await.expect("Failed to send message");
            }
        };
    }
}
