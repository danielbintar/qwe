use crate::core::object::user::User;

pub struct Content {
    pub current_user: Option<User>,
    pub current_chat: String,
    pub chat: Vec<String>,
}

impl Content {
    pub fn new() -> Self {
        Self{
            current_user: None,
            current_chat: String::from(""),
            chat: Vec::new()
        }
    }
}
