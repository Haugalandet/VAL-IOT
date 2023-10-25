use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub tokens: Option<String>
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: Default::default(),
            password: Default::default(),
            tokens: None
        }
    }
}