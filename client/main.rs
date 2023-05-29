use tokio::net::TcpStream;
use tokio::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to server");

    // Prompt for username and password
    let username = prompt("Enter username: ")?;
    let password = prompt("Enter password: ")?;

    // Send username to the server
    stream.write_all(username.as_bytes()).await?;

    // Send password to the server
    stream.write_all(password.as_bytes()).await?;

    // Read the authentication result from the server
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);

    Ok(())
}

fn prompt(message: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
