use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPosition {
    pub id: u32,
    x: u32,
    y: u32,
    pub sprite_id: Option<uuid::Uuid>,
}

impl UserPosition {
	pub fn new(id: u32, x: u32, y: u32) -> Self {
		Self {
			id,
			x,
			y,
			sprite_id: None,
		}
	}

	pub fn get_x(&self) -> f64 {
		self.x as f64 * 100.0
	}

	pub fn get_y(&self) -> f64 {
		self.y as f64 * 100.0
	}
}
