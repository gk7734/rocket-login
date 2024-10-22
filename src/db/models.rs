use bcrypt::{hash, verify};
use serde::{Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn hash_password(password: &str) -> String {
        hash(password, 4).expect("Failed to hash password")
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap_or(false)
    }
}