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
use chrono::Utc;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use crate::app::command::login_user_command::{LoginUserRepository, PasswordServiceTrait};
use crate::auth::auth_service_server::{AuthService, AuthServiceServer};
use crate::auth::{LoginReply, LoginRequest};
use crate::di::Container;
use crate::repository::postgres::user::user_repository::PostgresUserRepository;
use crate::service::password_service::PasswordService;

pub mod auth {
    tonic::include_proto!("auth");
}

pub struct MyAuth<R, S>
where
    R: LoginUserRepository + Send + Sync + 'static,
    S: PasswordServiceTrait + Send + Sync + 'static,
{
    container: Arc<Container<R, S>>,
}

impl<R, S> MyAuth<R, S>
where
    R: LoginUserRepository + Send + Sync + 'static,
    S: PasswordServiceTrait + Send + Sync + 'static,
{
    pub fn new(container: Arc<Container<R, S>>) -> Self {
        MyAuth { container }
    }
}

#[tonic::async_trait]
impl<R, S> AuthService for MyAuth<R, S>
where
    R: LoginUserRepository + Send + Sync + 'static,
    S: PasswordServiceTrait + Send + Sync + 'static,
{
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        // let req = request.get_ref();
        // let result = self
        //     .container
        //     .login_user_command
        //     .execute(req.email.to_string(), req.password.to_string())
        //     .await;
        //
        // match result {
        //     Ok(token) => Ok(Response::new(LoginReply { success: true, token: token})),
        //     Err(e) => Err(Status::internal(e.to_string())),
        // }

        Ok(Response::new(LoginReply { success: true, token: "1232".to_string()}))
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

    let container = Container::new(repo.clone(), password_service);
    let addr = "[::1]:50051".parse()?;
    let a = MyAuth::new(Arc::new(container));

    println!("Starting gRPC Server...");

    Server::builder()
        .add_service(AuthServiceServer::new(a))
        .serve(addr)
        .await?;



    Ok(())
}
