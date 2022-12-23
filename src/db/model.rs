use chrono::{DateTime};
use sha2::{Sha256, Digest};

pub enum UserStatus {
    Admin,
    Active,
    Pending,
    InActive,
}

pub struct User {
    email: String,
    status: UserStatus,
    password: String,
    hashSalt: String,
    hashIter: i64,
}

impl User {
    pub fn create_user(email: &str, password: &str)->User{
        let mut rng = thread_rng();
        let hashSalt = (0..32).
        let mut hasher = Sha256::new();
        hasher.update(password);
        let hashed_password = hasher.finalize();
        User { email: email.to_string(), status: UserStatus::Pending, password: format!("{:x}", hashed_password), hashSalt: (), hashIter: () }
    }
}