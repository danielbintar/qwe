use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u32,
    pub username: String,
    password: String,
}
