
use tonic::{Response, Request, Status};
use crate::services::api::health::{HealthRequest, HealthResponse};
use crate::services::api::health::check_server::Check;

#[derive(Default)]
pub struct CheckService {
}

#[tonic::async_trait]
impl Check for CheckService {
    async fn message(&self, request: Request<HealthRequest>) -> Result<Response<HealthResponse>, Status> {
        let req = request.get_ref();
        Ok(Response::new(HealthResponse {
            payload:  format!("{} pong", req.payload)
        }))
    }
}


