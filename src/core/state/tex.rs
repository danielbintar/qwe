use std::rc::Rc;

pub struct Tex<I: graphics::ImageSize> {
    player: Rc<I>,
}

impl<I: graphics::ImageSize> Tex<I> {
    pub fn new(player: Rc<I>) -> Self {
        Self {
            player,
        }
    }

    pub fn get_player(&self) -> &Rc<I> {
        &self.player
    }
}
