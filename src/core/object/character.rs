use crate::core::position::Position;

pub struct Character {
    pub position: Position,
}

impl Character {
    pub fn new() -> Self {
        Self{
            position: Position{
                x: 5,
                y: 5,
            }
        }
    }

    pub fn move_vertical(&mut self, x: u32, positive: bool) {
        if positive {
            self.position.x += x
        } else {
            if x > self.position.x {
                self.position.x = 0
            } else {
                self.position.x -= x
            }
        }
    }

    pub fn move_horizontal(&mut self, y: u32, positive: bool) {
        if positive {
            self.position.y += y
        } else {
            if y > self.position.y {
                self.position.y = 0
            } else {
                self.position.y -= y
            }
        }
    }
}
