mod services;
mod server;
mod pb;
mod mappings;

use std::net::SocketAddr;
use server::distances::run_grpc_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    println!("Server listening on {}", addr);

    run_grpc_server(addr).await?;

    Ok(())
}
