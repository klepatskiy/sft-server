use std::sync::Arc;
use async_trait::async_trait;
use shaku::Component;
use crate::application::QueryHandler;
use crate::infrastructure::service::jwt::JwtService;

pub struct VerifyTokenQuery {
    pub token: String,
}

#[derive(Component)]
#[shaku(interface = QueryHandler<VerifyTokenQuery, bool>)]
pub struct VerifyTokenQueryHandler {
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtService>,
}

#[async_trait]
impl QueryHandler<VerifyTokenQuery, bool> for VerifyTokenQueryHandler {
    async fn handle(&self, query: VerifyTokenQuery) -> Result<bool, Box<dyn std::error::Error>> {
        let result = self.jwt_service.verify_jwt(&query.token).await;
        
        Ok(result.is_ok())
    }
}
