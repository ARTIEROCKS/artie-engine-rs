mod services;
mod server;
mod pb;
mod mappings;


use server::distances::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Starting gRPC server...");
    start_server().await?;
    Ok(())
}
