use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;



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

    Ok(())
}