mod api;



use tonic::{Response, Request, Status};
use crate::api::health::check_server::{Check, CheckServer};
use crate::api::health::{HealthRequest, HealthResponse};

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

pub fn health_server() -> CheckServer<CheckService> {
    let srv = CheckService::default();
    CheckServer::with_interceptor(srv, intercept)
}


fn intercept(req: Request<()>) ->Result<Request<()>, Status> {
    println!("{:?}", req);
    Ok(req)
}

