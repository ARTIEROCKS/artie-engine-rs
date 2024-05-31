use tonic::transport::Server;
use crate::pb::artie_distances::artie_common_service_server::ArtieCommonServiceServer;
use crate::services::distances::MyArtieCommonService;

pub async fn run_grpc_server(addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let artie_common_service = MyArtieCommonService::default();

    Server::builder()
        .add_service(ArtieCommonServiceServer::new(artie_common_service))
        .serve(addr)
        .await?;

    Ok(())
}
