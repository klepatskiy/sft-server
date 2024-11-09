use tonic::{Status};
use std::sync::Arc;
use async_trait::async_trait;
use tonic::body::BoxBody;
use tonic::codegen::http::{HeaderValue, Request};
use tonic_middleware::RequestInterceptor;
use tower::{ServiceExt};
use crate::app::error::AppError;
use crate::domain::user::user_token::{UserWithToken};

#[async_trait]
pub trait TokenServiceTrait: Send + Sync {
    async fn get_user_by_token(&self, user_token: &str) -> Result<UserWithToken, AppError>;
}


#[derive(Clone)]
pub struct AuthInterceptor {
    pub token_service: Arc<dyn TokenServiceTrait>,
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, mut req: Request<BoxBody>) -> Result<Request<BoxBody>, Status> {
        if req.uri().path() == "/auth.AuthService/Login" {
            // Пропускаем аутентификацию для метода "Login"
            return Ok(req);
        }

        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                let user_with_token = self
                    .token_service
                    .get_user_by_token(token)
                    .await;

                // // Set user id in header, so it can be used in grpc services through tonic::Request::metadata()
                // let user_id_header_value = HeaderValue::from_str(&user_with_token.to_string())
                //     .map_err(|_e| Status::internal("Failed to convert user_id to header value"))?;
                // req.headers_mut().insert("user_id", user_id_header_value);

                // req.extensions_mut().insert(user_with_token.unwrap().user);

                Ok(req)
            }
            _ => Err(Status::unauthenticated("Unauthenticated")),
        }
    }
}