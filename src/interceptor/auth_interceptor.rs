use tonic::{Status};
use std::sync::Arc;
use async_trait::async_trait;
use tonic::body::BoxBody;
use tonic::codegen::http::{Request};
use tonic_middleware::RequestInterceptor;
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
            return Ok(req);
        }

        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                let user_with_token = match self.token_service.get_user_by_token(token).await {
                    Ok(user_with_token) => user_with_token,
                    Err(AppError::InvalidCredentials) => {
                        return Err(Status::unauthenticated("Invalid token"));
                    },
                    Err(_) => {
                        return Err(Status::internal("Failed to retrieve user token"));
                    }
                };

                req.extensions_mut().insert(user_with_token.user);

                Ok(req)
            }
            _ => Err(Status::unauthenticated("Unauthenticated")),
        }
    }
}
