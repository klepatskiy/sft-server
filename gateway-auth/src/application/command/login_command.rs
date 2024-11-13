use std::sync::Arc;
use async_trait::async_trait;
use shaku::Component;
use crate::application::CommandHandler;
use crate::infrastructure::service::jwt::JwtService;

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
    async fn handle(&self, command: LoginCommand) -> Result<(String, String), Box<dyn std::error::Error>> {
        let (access_token, refresh_token) = self.jwt_service.create_jwt(&command.email).await?;
        
        Ok((access_token, refresh_token))
    }
}
