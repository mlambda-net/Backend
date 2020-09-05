use crate::services::health_server::CheckService;
use tonic::{Request, Status};
use crate::services::api::health::check_server::CheckServer;

mod health_server;
mod api;

pub fn health_server() -> CheckServer<CheckService> {
    let srv = CheckService::default();
    CheckServer::with_interceptor(srv, intercept)
}


fn intercept(req: Request<()>) ->Result<Request<()>, Status> {
    println!("{:?}", req);
    Ok(req)
}