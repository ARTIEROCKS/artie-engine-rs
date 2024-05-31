use tonic::{Request, Response, Status};
use crate::pb::artie_distances::{
    artie_common_service_server::ArtieCommonService,
    ArtieDistance, DistanceRequest,
};

// Implement the service trait
pub struct MyArtieCommonService;

impl Default for MyArtieCommonService {
    fn default() -> Self {
        MyArtieCommonService
    }
}

#[tonic::async_trait]
impl ArtieCommonService for MyArtieCommonService {
    async fn distance_calculation(
        &self,
        request: Request<DistanceRequest>,
    ) -> Result<Response<ArtieDistance>, Status> {
        let _req = request.into_inner();

        // Placeholder for actual logic
        let response = ArtieDistance {
            family_distance: 0.0,
            block_distance: 0.0,
            position_distance: 0.0,
            input_distance: 0.0,
            total_distance: 0.0,
            workspace_adjustments: None, // Adjust as per your proto definition
        };

        Ok(Response::new(response))
    }
}
