pub mod container;

use std::sync::Arc;
use crate::app::command::login_user_command::{LoginUserCommandTrait};

// Интерфейс для DI контейнера
pub trait DIContainer: Send + Sync {
    fn login_user_command(&self) -> Arc<dyn LoginUserCommandTrait>;
}

// Основной контейнер
pub struct Container {
    pub login_user_command: Arc<dyn LoginUserCommandTrait>,
    // другие зависимости...
}

impl DIContainer for Container {
    fn login_user_command(&self) -> Arc<dyn LoginUserCommandTrait> {
        Arc::clone(&self.login_user_command)
    }
}
