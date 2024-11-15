use std::sync::Arc;
use tonic::{Request, Status};
use tonic::service::Interceptor;
use shaku::HasComponent;
use crate::application::command::refresh_token_command::RefreshTokenCommand;
use crate::di_container::UserContainer;
use crate::application::query::verify_token_query::{VerifyTokenQuery};
use crate::application::{CommandHandler, QueryHandler};

pub struct AuthInterceptor {
    verify_token_query_handler: Arc<dyn QueryHandler<VerifyTokenQuery, bool>>,
}

impl AuthInterceptor {
    pub fn new(container: UserContainer) -> Self {
         let verify_token_query_handler: &dyn QueryHandler<VerifyTokenQuery, bool> =
            container.resolve_ref();
        
        let b = Arc::new(verify_token_query_handler);
        Self { verify_token_query_handler : b }
    }
}

impl Interceptor for AuthInterceptor {
    // todo это не будет рабатоать переписать нужно пока просто хочу запушить все в мастер
    
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Извлекаем токен из метаданных
        let token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(Status::unauthenticated("Missing token"))?;

        // Создаем запрос для проверки токена
        let query = VerifyTokenQuery {
            token: token.to_string(),
        };

        // Используем VerifyTokenQueryHandler для обработки запроса
        let is_valid = self
            .verify_token_query_handler
            .handle(query)
            .await
            .map_err(|_| Status::unauthenticated("Invalid token"))?;

        if !is_valid {
            return Err(Status::unauthenticated("Token verification failed"));
        }

        // Добавляем `user_id` в метаданные (предположительно, он возвращается в QueryHandler)
        request.extensions_mut().insert(query.token.clone());
        Ok(request)
    }
}