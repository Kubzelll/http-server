mod parser;

use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use parser::parse_request;
use std::fs;

fn read_file(file_path: &str) -> Result<String, std::io::Error>{
    let content = fs::read_to_string(file_path);
    content
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "127.0.0.1:8080".to_string();
    println!("Server is listening on {}", &addr);
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
    let file_path = format!("html{}", path);
    println!("{}", file_path);
    let file = read_file(&file_path).expect("File not found");
    let response = format!(
      "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        file.len(),
        file
    );
    socket.write_all(response.as_bytes()).await?;
    Ok(())
}