use std::net::SocketAddr;
use std::time::Duration;
use tokio_modbus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = "127.0.0.1:5020".parse().unwrap();
    println!("Connecting client to {}", socket_addr);
    let mut ctx = tcp::connect(socket_addr).await?;
    
    println!("Client connected. Waiting a bit for timeout...");
    // We expect a timeout since no RTU server is answering on pty2, but the TCP Bridge should properly handle it
    let res = tokio::time::timeout(Duration::from_millis(500), ctx.read_coils(0, 1)).await;
    println!("Client result: {:?}", res);
    
    Ok(())
}
