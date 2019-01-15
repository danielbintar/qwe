use crate::core::object::user::User;

pub struct Content {
    pub current_user: Option<User>,
}

impl Content {
    pub fn new() -> Self {
        Self{
            current_user: None,
        }
    }
}
