use std::sync::Arc;
use async_trait::async_trait;
use shaku::Component;
use crate::application::CommandHandler;
use crate::application::error::AppError;
use crate::infrastructure::service::jwt::JwtService;

pub struct RefreshTokenCommand {
    pub refresh_token: String,
}

#[derive(Component)]
#[shaku(interface = CommandHandler<RefreshTokenCommand, String>)]
pub struct RefreshTokenCommandHandler {
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
}

#[async_trait]
impl CommandHandler<RefreshTokenCommand, String> for RefreshTokenCommandHandler {
    async fn handle(&self, command: RefreshTokenCommand) -> Result<String, AppError> {
        // let token_data = self.jwt_service.verify_refresh_token(&command.refresh_token).await?;
        // Ok(token_data)
        Ok("".to_string())
    }
}
