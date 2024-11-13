use tonic::{Request, Response, Status};
use async_trait::async_trait;
use jsonwebtoken::TokenData;
use shaku::HasComponent;
use crate::di_container::AuthContainer;

pub mod auth_proto {
    tonic::include_proto!("auth");
}

use auth_proto::auth_service_server::AuthService;
use crate::application::command::login_command::{LoginCommand};
use crate::application::command::refresh_token_command::RefreshTokenCommand;
use crate::application::CommandHandler;
use crate::auth_service::auth_proto::{LoginRequest, LoginResponse, RefreshRequest, RefreshResponse};
use crate::domain::jwt::jwt_model::RefreshClaims;

pub struct AuthServiceImpl {
    container: AuthContainer,
}

impl AuthServiceImpl {
    pub fn new(container: AuthContainer) -> Self {
        AuthServiceImpl { container }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let data = request.into_inner();
        let command = LoginCommand {
            email: data.email,
            password: data.password,
        };

        let handler: &dyn CommandHandler<LoginCommand, (String, String)> =
            self.container.resolve_ref();

        let (access_token, refresh_token) = handler
            .handle(command)
            .await
            .map_err(|_| Status::internal("Error creating tokens"))?;
        
        Ok(Response::new(LoginResponse {
            access_token,
            refresh_token,
        }))
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let refresh_data = request.into_inner();
        let command = RefreshTokenCommand {
            refresh_token: refresh_data.refresh_token,
        };

        let handler: &dyn CommandHandler<RefreshTokenCommand, (TokenData<RefreshClaims>)> =
            self.container.resolve_ref();

        let token = handler
            .handle(command)
            .await
            .map_err(|_| Status::internal("Error creating tokens"))?;
        
        Ok(Response::new(RefreshResponse {
            access_token: "new_access_token".to_string(),
            refresh_token: "new_refresh_token".to_string(),
        }))
    }
}