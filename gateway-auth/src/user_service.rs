use crate::auth_service::auth_proto::auth_service_server::AuthService;
use crate::di_container::UserContainer;
use crate::user_service::auth_proto::user_service_server::UserService;
use crate::user_service::auth_proto::{GetCurrentUserRequest, GetCurrentUserResponse, GetUserRequest, GetUserResponse};
use async_trait::async_trait;
use tonic::{Request, Response, Status};

pub mod auth_proto {
    tonic::include_proto!("user");
}

pub struct UserServiceImpl {
    container: UserContainer,
}

impl UserServiceImpl {
    pub fn new(container: UserContainer) -> Self {
        UserServiceImpl { container }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        Ok(Response::new(GetUserResponse {
            id: "1".to_string(),
        }))
    }

    async fn get_current_user(
        &self,
        request: Request<GetCurrentUserRequest>,
    ) -> Result<Response<GetCurrentUserResponse>, Status> {
        Ok(Response::new(GetCurrentUserResponse {
            id: "1".to_string(),
        }))
    }
}
