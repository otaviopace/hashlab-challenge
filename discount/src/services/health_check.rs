use crate::ports::tonic::{Empty, HealthService, ServingStatus, StatusResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct HealthCheckGrpc {}

#[tonic::async_trait]
impl HealthService for HealthCheckGrpc {
    async fn check(&self, request: Request<Empty>) -> Result<Response<StatusResponse>, Status> {
        println!("Request: {:?}", request);

        let response = StatusResponse {
            status: ServingStatus::Serving.into(),
        };

        Ok(Response::new(response))
    }
}
