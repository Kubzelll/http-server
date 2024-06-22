mod parser;

use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use parser::parse_request;
use std::fs;
use std::io::Read;
use mime_guess::from_path;

fn get_mime(file_path: &String) -> Option<String>{
    from_path(file_path).first_raw().map(|mime| mime.to_string())
}
fn sanitize_path(input_path: &str) -> bool {
    // Check for path traversal
    if input_path.contains("..") {
        return false;
    }
    return true
}
fn read_file(file_path: &String) -> Result<String, std::io::Error>{
    let content = fs::read_to_string(file_path);
    content
}
fn read_file_bytes(file_path: &String) -> Result<Vec<u8>, std::io::Error>{
    let content = fs::read(file_path);
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
    match sanitize_path(&file_path) {
       true => {
           println!("Sanitized path: {:?}", file_path);
//           let mime_type = get_mime(&file_path).unwrap().to_string();
           let mut mime_type = String::new();
           let mut mime_type_display = get_mime(&file_path);
           if mime_type_display == Option::None{
               let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 21\r\n\r\n<h>File not found</h>";
               socket.write(response.as_bytes()).await?;
               return Ok(())
           }else {
               mime_type = mime_type_display.unwrap().to_string();
           }
           if mime_type.contains("image") == true{
               println!("Handling as image");
               //handle the file as image
               match read_file_bytes(&file_path) {
                   Ok(image_bytes) =>{
                       let response = format!(
                           "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                           mime_type,
                           image_bytes.len(),
                       );
                       socket.write(response.as_bytes()).await?;
                       socket.write(&image_bytes).await?;
                       return  Ok(())
                   }
                   Err(e) => {
                       let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 21\r\n\r\n<h>File not found</h>";
                       println!("{}", e);
                       socket.write(response.as_bytes()).await?;
                        return Ok(())
                   }
               }
           }
            match read_file(&file_path){
                Ok(file) => {
                    let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    mime_type,
                    file.len(),
                    file.to_string()
                    );
                    println!("{}", response);
                    socket.write(response.as_bytes()).await?;

                }
                Err(e) => {
                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 21\r\n\r\n<h>File not found</h>";
                    println!("{}", e);
                    socket.write(response.as_bytes()).await?;
                }
            }
        }
        false => {
            let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 21\r\n\r\n<h>File not found</h>";
            socket.write(response.as_bytes()).await?;
        }
    }


    Ok(())
}