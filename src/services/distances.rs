use artie_common::structure::Workspace;
use artie_common::operations::artie_distance::artie_distance;
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

        let req = request.into_inner();

        let workspace: Workspace = match req.workspace {
            Some(ws) => ws.into(),
            None => return Err(Status::invalid_argument("workspace is missing")),
        };

        let solution: Workspace = match req.solution {
            Some(sol) => sol.into(),
            None => return Err(Status::invalid_argument("solution is missing")),
        };

        let artie_distance: ArtieDistance = artie_distance(&workspace, &solution).into();

        Ok(Response::new(artie_distance))
    }
}
