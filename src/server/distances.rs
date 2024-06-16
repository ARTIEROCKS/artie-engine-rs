use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use crate::pb::artie_distances::artie_common_service_server::ArtieCommonServiceServer;
use crate::services::distances::MyArtieCommonService;
use tokio::signal;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let artie_common_service = MyArtieCommonService::default();

    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(crate::pb::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(ArtieCommonServiceServer::new(artie_common_service))
        .add_service(reflection_service)
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    // Captura Ctrl+C
    signal::ctrl_c()
        .await
        .expect("error configuring Ctrl+C");

    println!("Ctrl+C received, stopping the server...");
}
