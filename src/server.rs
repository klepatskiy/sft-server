mod app;
mod config;
mod di;
mod domain;
mod repository;
mod service;

use std::env;
use dotenv::dotenv;
use std::sync::Arc;
use argon2::Argon2;
use tonic::{transport::Server, Request, Response, Status};
use crate::app::command::login_user_command::{LoginUserRepository, PasswordServiceTrait};
use crate::auth::auth_service_server::{AuthService, AuthServiceServer};
use crate::auth::{LoginReply, LoginRequest};
use crate::di::{DIContainer};
use crate::di::container::Container;
use crate::repository::postgres::user::user_repository::PostgresUserRepository;
use crate::service::password_service::PasswordService;

pub mod auth {
    tonic::include_proto!("auth");
}

pub struct MyAuth<C: DIContainer> {
    container: Arc<C>,
}

impl<C: DIContainer> MyAuth<C> {
    pub fn new(container: Arc<C>) -> Self {
        MyAuth { container }
    }
}

#[tonic::async_trait]
impl<C: DIContainer> AuthService for MyAuth<C> {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        let req = request.get_ref();
        let result = self
            .container
            .login_user_command()
            .execute(req.email.to_string(), req.password.to_string())
            .await;

        match result {
            Ok(token) => Ok(Response::new(LoginReply { success: true, token })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = config::create_pool().await.expect("Failed to create pool");
    let pool = Arc::new(pool);
    let repo = PostgresUserRepository::new(Arc::clone(&pool));
    let salt = env::var("PASSWORD_SALT").unwrap_or_else(|_| "default_salt_value".to_string());
    let password_service = PasswordService::new(salt, Argon2::default());

    let container = Container::new(Arc::new(repo), Arc::new(password_service));
    let addr = "[::1]:50051".parse()?;
    let auth_service = MyAuth::new(Arc::new(container));

    println!("Starting gRPC Server...");

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
