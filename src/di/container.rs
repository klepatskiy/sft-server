use std::sync::Arc;
use crate::app::command::login_user_command::{LoginUserCommand, LoginUserRepository, PasswordServiceTrait, LoginUserCommandTrait};
use crate::di::DIContainer;

pub struct Container {
    login_user_command: Arc<dyn LoginUserCommandTrait>,
}

impl Container {
    pub fn new(
        login_user_repository: Arc<dyn LoginUserRepository>,
        password_service: Arc<dyn PasswordServiceTrait>,
    ) -> Self {
        let login_user_command = Arc::new(LoginUserCommand::new(
            login_user_repository,
            password_service,
        ));

        Container {
            login_user_command,
        }
    }
}

impl DIContainer for Container {
    fn login_user_command(&self) -> Arc<dyn LoginUserCommandTrait> {
        Arc::clone(&self.login_user_command)
    }
}
