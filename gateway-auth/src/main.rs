mod di_container;
mod infrastructure;
mod auth_service;
mod application;
mod domain;

use dotenv::dotenv;
use std::env;
use tonic::transport::Server;
use di_container::AuthContainer;
use crate::auth_service::auth_proto::auth_service_server::AuthServiceServer;
use crate::auth_service::AuthServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set").into_bytes();
    let refresh_secret_key = env::var("REFRESH_SECRET_KEY").expect("REFRESH_SECRET_KEY must be set").into_bytes();

    let container = AuthContainer::new(secret_key, refresh_secret_key);

    let auth_service = AuthServiceImpl::new(container);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve("[::1]:50052".parse().unwrap())
        .await?;

    Ok(())
}
