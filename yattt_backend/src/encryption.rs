use crate::error::AppError;
use bcrypt::BcryptError;

pub trait PasswordEncrypter {
    fn hash_password(password: &str) -> Option<String>;
    fn verify_password(password: &str, hash: &str) -> bool;
}

impl From<BcryptError> for AppError {
    fn from(_: BcryptError) -> Self {
        AppError::InternalServerError
    }
}

#[derive(Clone)]
pub struct BcryptPasswordEncrypter;

impl PasswordEncrypter for BcryptPasswordEncrypter {
    fn hash_password(password: &str) -> Option<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).ok()
    }

    fn verify_password(password: &str, hash: &str) -> bool {
        bcrypt::verify(password, hash).is_ok()
    }
}