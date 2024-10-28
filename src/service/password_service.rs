use std::sync::Arc;
use argon2::{self, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
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
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, AppError> {
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

    pub fn verify_password(&self, hash: &str, password: &str) -> Result<bool, PasswordServiceError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| PasswordServiceError::VerificationError)?;
        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| PasswordServiceError::VerificationError)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> PasswordService {
        PasswordService::new("test_salt".to_string(), Argon2::default())
    }

    #[test]
    fn test_hash_password_success() {
        let password_service = setup();
        let password = "secure_password";
        let hash = password_service.hash_password(password);
        assert!(hash.is_ok(), "Хэширование пароля не должно завершаться с ошибкой");

        let second_hash = password_service.hash_password(password);
        assert!(second_hash.is_ok(), "Второе хэширование также должно завершиться успешно");

        // Оба хэша должны быть одинаковыми, так как используется фиксированная соль
        assert_eq!(hash.unwrap(), second_hash.unwrap(), "Хэши должны быть одинаковыми для одного пароля и фиксированной соли");
    }

    #[test]
    fn test_verify_password_success() {
        let password_service = setup();
        let password = "secure_password";
        let hash = password_service.hash_password(password).unwrap();
        let is_valid = password_service.verify_password(&hash, password);
        assert!(is_valid.is_ok(), "Пароль должен быть верным");
        assert!(is_valid.unwrap(), "Пароль должен совпадать с хэшем");
    }

    #[test]
    fn test_verify_password_invalid() {
        let password_service = setup();
        let password = "secure_password";
        let hash = password_service.hash_password(password).unwrap();
        let wrong_password = "wrong_password";
        let is_valid = password_service.verify_password(&hash, wrong_password);
        assert!(is_valid.is_ok(), "Проверка пароля не должна завершаться с ошибкой");
        assert!(!is_valid.unwrap(), "Пароль не должен совпадать с хэшем");
    }

    #[test]
    fn test_invalid_hash_format() {
        let password_service = setup();
        let password = "secure_password";
        let invalid_hash = "invalid_hash_format";
        let is_valid = password_service.verify_password(invalid_hash, password);
        assert!(is_valid.is_err(), "Неверный формат хэша должен вызывать ошибку");
    }
}