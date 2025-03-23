use tokio::{net::TcpListener, net::TcpStream, io::AsyncWriteExt, io::AsyncReadExt};
use sha1::{Sha1, Digest};
use base64::{Engine as _, engine::general_purpose};
use std::error::Error;



pub struct WebSocket{
    stream: TcpStream
}


impl WebSocket {
    pub async fn from_tcp_stream(stream: TcpStream) -> Self {
        WebSocket { stream }
    }

    pub async fn validate_connection(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer = vec![0; 1024];

        let size = self.stream.read(&mut buffer).await?;
        if size == 0 { return Err("No Data received".into()); }
        
        let request  = String::from_utf8(buffer)?;
        if !request.starts_with("GET") { return Err("Not a valid GET request".into()); }

        let response = self.handshake(request).await?;

        self.stream.write_all(response.as_bytes()).await?;
        self.stream.flush().await?;

        Ok(())
    }

    async fn handshake(&mut self, request: String) -> Result<String, Box<dyn Error>> {
        let key = request
        .lines()
        .find(|line| line.starts_with("Sec-WebSocket-Key:"))
        .and_then(|line| line.split(':').nth(1))
        .unwrap_or("")
        .trim();

        if key.is_empty() { return Err("No Sec-WebSocket-Key found".into())}

        let guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

        let mut hasher = Sha1::new();
        hasher.update(key.as_bytes());
        hasher.update(guid.as_bytes());

        let accept_key = general_purpose::STANDARD.encode(hasher.finalize());

        let response = format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
            Upgrade: websocket\r\n\
            Connection: upgrade\r\n\
            Sec-WebSocket-Accept: {}\r\n\r\n",
            accept_key
        );
        
        Ok(response)
    }
    
    pub async fn handle_active_connection(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}


async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut ws: WebSocket = WebSocket::from_tcp_stream(stream).await;
    ws.validate_connection().await?;
    ws.handle_active_connection().await?;
    println!("Connection successful");
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?; 

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error {}:{}", addr, e);
            }
        });  
    }
}