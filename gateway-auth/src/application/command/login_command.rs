use std::sync::Arc;
use async_trait::async_trait;
use shaku::Component;
use uuid::Uuid;
use crate::application::CommandHandler;
use crate::application::error::AppError;
use crate::infrastructure::service::jwt::{JwtService, JwtServiceError};
use std::str::FromStr;

pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Component)]
#[shaku(interface = CommandHandler<LoginCommand, (String, String)>)]
pub struct LoginCommandHandler {
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
}

#[async_trait]
impl CommandHandler<LoginCommand, (String, String)> for LoginCommandHandler {
    async fn handle(&self, command: LoginCommand) -> Result<(String, String), AppError> {
        // todo тут следует подкинуть юзер сервис и проверить пароль и только потом генерить токены
        // todo сейчас я в реквесте вместо email проикидываю uuid
        let uuid = Uuid::from_str(&*command.email).map_err(|_| AppError::SomeError)?;
        
        let access_token = match self.jwt_service.create_access_token(uuid).await {
            Ok(token) => token,
            Err(err) => match err {
                JwtServiceError::TokenExpireError => {
                    return Err(AppError::CreateTokenError);
                }
                _ =>  return Err(AppError::SomeError)
            }
        };
        let refresh_token = match self.jwt_service.create_refresh_token(uuid).await {
            Ok(token) => token,
            Err(err) => match err {
                _ =>  return Err(AppError::SomeError)
            }
        };
        
        Ok((access_token, refresh_token))
    }
}
