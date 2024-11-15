use shaku::{module};
use crate::application::command::login_command::LoginCommandHandler;
use crate::application::command::refresh_token_command::RefreshTokenCommandHandler;
use crate::application::query::verify_token_query::VerifyTokenQueryHandler;
use crate::infrastructure::service::jwt::{JwtService, JwtServiceImpl};

module! {
    pub AuthContainer {
        components = [
            JwtServiceImpl,
            LoginCommandHandler,
            RefreshTokenCommandHandler,
            VerifyTokenQueryHandler
        ],
        providers = []
    }
}

impl AuthContainer {
    pub fn new(secret_key: Vec<u8>, refresh_secret_key: Vec<u8>) -> Self {
        let jwt_service = JwtServiceImpl::new(secret_key, refresh_secret_key);
        AuthContainer::builder()
            .with_component_override(Box::new(jwt_service) as Box<dyn JwtService>)
            .build()
    }
}

module! {
    pub UserContainer {
        components = [
            JwtServiceImpl,
            LoginCommandHandler,
            RefreshTokenCommandHandler,
            VerifyTokenQueryHandler
        ],
        providers = []
    }
}

impl UserContainer {
    pub fn new(secret_key: Vec<u8>, refresh_secret_key: Vec<u8>) -> Self {
        let jwt_service = JwtServiceImpl::new(secret_key, refresh_secret_key);
        UserContainer::builder()
            .with_component_override(Box::new(jwt_service) as Box<dyn JwtService>)
            .build()
    }
}
