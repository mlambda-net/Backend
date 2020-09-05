mod services;


use crate::services::health_server;
use tonic::transport::Server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse().unwrap();
    println!("Server listening at 8080");
    Server::builder()
        .add_service(health_server())
        .serve(addr)
        .await?;
    Ok(())
}

