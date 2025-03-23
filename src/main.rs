use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use rand::Rng;
use base64::{Engine as _, engine::general_purpose};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    
    println!("Connecting to {}", addr);
    let mut stream = TcpStream::connect(addr).await?;

    let req_header = generate_request_header(addr);
    println!("Request header:\n{}", req_header);

    stream.write_all(req_header.as_bytes()).await?;
    stream.flush().await?;

    let mut response = vec![0; 1024];
    let n = stream.read(&mut response).await?;
    println!("Response:\n{}", String::from_utf8_lossy(&response[..n]));

    Ok(())
}

fn generate_request_header(host: &str)  -> String {
    format!(
        "GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        Upgrade: websocket\r\n\
        Connection: Upgrade\r\n\
        Sec-WebSocket-Key: {}\r\n\
        Sec-WebSocket-Version: 13\r\n\r\n",
        host, generate_sec_websocket_key()
    )
}

fn generate_sec_websocket_key() -> String {
    let mut random_bytes: [u8; 16] = [0u8; 16];
    rand::rng().fill(&mut random_bytes);
    general_purpose::STANDARD.encode(&random_bytes)
}

//fn connection_upgrade_client() {
    // header
    // Connection: upgrade
    // Upgrade: websocket
    // Sec-WebSocket-Key: <random_key>
//}

//fn server_upgrade_response() {
    // 101 <- succeed
    // header response
    // Connection: upgrade
    // Upgrade: websocket
    // Sec-WebSocket-Accept: <accept_response> // Server appends a specific GUID to this key
    // 258eafa5-e914-47da-95ca-c5ab0dc85b11
    // Computes the Sha-1 hash
    // Sec-WebSocket-Accept ist the base64 encoding of the hash
//}