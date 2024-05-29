mod parser;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use parser::parse_request;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (mut _socket, ip) = listener.accept().await?;
        tokio::spawn(
            async move{
                if let Err(e) = handle_client(&mut _socket).await {
                    eprintln!("Failed to handle client {:?}", e);
                }else{
                    println!("Thread created {:?}", ip);
                }
            }
        );
    }
}

async fn handle_client(socket: &mut TcpStream) -> Result<(), Box<dyn  std::error::Error>>{
    let mut buffer = [0; 2048];

    let n = socket.read(&mut buffer).await?;
    let Ok((method, path, headers)) = parse_request(&buffer[..n] )else { return Ok(()) };
    let response_body = format!(
        "Method: {}\nPath: {}\nHeaders: {:?}",
        method, path, headers
    );
    let response = format!(
      "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    println!("{}", path);
    socket.write_all(response.as_bytes()).await?;
    Ok(())
}