use std::sync::Arc;
use argon2::{self, Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use thiserror::Error;
use crate::app::command::login_user_command::PasswordServiceTrait;
use crate::app::error::AppError;

#[derive(Debug, Error)]
pub enum PasswordServiceError {
    #[error("Password hashing error")]
    HashingError,
    #[error("Password verification error")]
    VerificationError,
}

pub struct PasswordService {
    salt: String,
    argon2: Arc<Argon2<'static>>,
}

impl PasswordServiceTrait for PasswordService {
    fn verify_password(&self, hash: &str, password: &str) -> Result<(), AppError> {
        self.verify_password(hash, password).map_err(|_| AppError::InvalidCredentials)
    }
}

impl PasswordService {
    pub fn new(salt: String, argon2: Argon2<'static>) -> Self {
        PasswordService { salt, argon2: Arc::new(argon2) }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, PasswordServiceError> {
        let salt = SaltString::encode_b64(self.salt.as_bytes()).unwrap();
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| PasswordServiceError::HashingError)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, hash: &str, password: &str) -> Result<(), PasswordServiceError> {
        let hash_password = self.hash_password(password)?;
        let hash_password_str = hash_password.as_str();

        if hash_password_str != hash {
            return Err(PasswordServiceError::VerificationError);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::Argon2;

    // Утилита для создания экземпляра PasswordService с фиксированной солью для тестирования
    fn setup() -> PasswordService {
        PasswordService::new("test_salt".to_string(), Argon2::default())
    }

    #[test]
    fn test_hash_password_success() {
        let password_service = setup();
        let password = "secure_password";

        // Проверка успешного хэширования пароля
        let hash_result = password_service.hash_password(password);
        assert!(hash_result.is_ok(), "Хэширование пароля должно завершаться без ошибок");

        // Повторное хэширование для проверки использования фиксированной соли
        let hash_again = password_service.hash_password(password);
        assert!(hash_again.is_ok(), "Второе хэширование должно завершаться успешно");

        // Проверка совпадения хэшей при одинаковом пароле и соли
        assert_eq!(hash_result.unwrap(), hash_again.unwrap(), "Хэши должны совпадать при фиксированной соли");
    }

    #[test]
    fn test_verify_password_success() {
        let password_service = setup();
        let password = "secure_password";

        // Хэшируем пароль
        let hash = password_service.hash_password(password).unwrap();

        // Проверка успешной верификации правильного пароля
        let verification_result = password_service.verify_password(&hash, password);
        assert!(verification_result.is_ok(), "Верификация пароля должна завершаться без ошибок");
    }

    #[test]
    fn test_verify_password_invalid() {
        let password_service = setup();
        let password = "secure_password";
        let hash = password_service.hash_password(password).unwrap();
        let wrong_password = "wrong_password";

        // Проверка верификации с неверным паролем
        let verification_result = password_service.verify_password(&hash, wrong_password);
        assert!(verification_result.is_err(), "Верификация с неверным паролем должна завершаться ошибкой");
    }

    #[test]
    fn test_invalid_hash_format() {
        let password_service = setup();
        let password = "secure_password";
        let invalid_hash = "invalid_hash_format";

        // Проверка верификации с неверным форматом хэша
        let verification_result = password_service.verify_password(invalid_hash, password);
        assert!(verification_result.is_err(), "Верификация с неправильным форматом хэша должна завершаться ошибкой");
    }
}
