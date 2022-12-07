use tokio::net::{TcpStream, TcpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use chatchatchat::models;


#[tokio::main]
async fn main() {
    let socket = TcpSocket::new_v4().expect("Failed to create socket");
    let server_url = env!("SERVER_URL").parse().unwrap();
    let mut stream = socket.connect(server_url).await.expect("Failed to connect");

    let message = models::NetworkPayload::MESSAGE(
        models::MessagePayload {
            content: "".to_string(),
            author: models::AuthorIdentity {
                nickname: "".to_string()
            }
        }
    );
    let mut serialized_message = serde_json::to_vec(&message).expect("Failed to encode");
    println!("Packet size: {}", serialized_message.len());
    stream.write_u32(serialized_message.len() as u32).await;
    stream.write(&serialized_message).await;
    stream.flush().await.expect("Failed to flush");

    loop {}
}
