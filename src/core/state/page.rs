use crate::core::state::State;

pub enum Page {
    Login,
    Menu
}

impl Page {
    pub fn new() -> Page {
        Page::Login
    }

    pub fn login(&mut self) {
        *self = Page::Menu;
    }
}

impl State for Page {
    fn available(&self, next_state: impl State) -> bool {
        true
    }
}
