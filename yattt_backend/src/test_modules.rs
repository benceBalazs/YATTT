pub struct TestEncrypter;

impl yattt_backend::encryption::PasswordEncrypter for TestEncrypter {
    fn hash_password(password: &str) -> Option<String> {
        Some(password.to_string())
    }

    fn verify_password(password: &str, hash: &str) -> bool {
        password == hash
    }
}