use std::sync::Arc;
use async_trait::async_trait;
use jsonwebtoken::TokenData;
use shaku::Component;
use crate::application::CommandHandler;
use crate::domain::jwt::jwt_model::RefreshClaims;
use crate::infrastructure::service::jwt::JwtService;

pub struct RefreshTokenCommand {
    pub refresh_token: String,
}

#[derive(Component)]
#[shaku(interface = CommandHandler<RefreshTokenCommand, TokenData<RefreshClaims>>)]
pub struct RefreshTokenCommandHandler {
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
}

#[async_trait]
impl CommandHandler<RefreshTokenCommand, TokenData<RefreshClaims>> for RefreshTokenCommandHandler {
    async fn handle(&self, command: RefreshTokenCommand) -> Result<TokenData<RefreshClaims>, Box<dyn std::error::Error>> {
        let token_data = self.jwt_service.verify_refresh_token(&command.refresh_token).await?;
        Ok(token_data)
    }
}
