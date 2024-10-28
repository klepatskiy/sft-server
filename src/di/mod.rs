use crate::app::command::login_user_command::{LoginUserCommand, LoginUserRepository, PasswordServiceTrait};

pub struct Container<R, S>
where
    R: LoginUserRepository,
    S: PasswordServiceTrait,
{
    pub login_user_command: LoginUserCommand<R, S>,
}

impl<R, S> Container<R, S>
where
    R: LoginUserRepository,
    S: PasswordServiceTrait + 'static,
{
    pub fn new(repository_command: R, password_service: S) -> Self {
        let login_user_command = LoginUserCommand::new(repository_command, password_service);

        Container {
            login_user_command,
        }
    }
}