use crate::core::position::Position;

pub struct Character {
    pub position: Position,
}

impl Character {
    pub fn new() -> Self {
        Self{
            position: Position{
                x: 100.0,
                y: 100.0,
            }
        }
    }
}