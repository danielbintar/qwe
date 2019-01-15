mod login;
mod character_selection;
mod content;

use self::login::Login;
use self::character_selection::CharacterSelection;
use self::content::Content;

pub struct State {
    current_page: Page,
    content: Content,

    character_selection_page: CharacterSelection,
    login_page: Login,
}

enum Page {
    Login,
    CharacterSelection,
}

impl State {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Self {
            login_page: Login::new(ui),
            current_page: Page::Login,
            content: Content::new(),
            character_selection_page: CharacterSelection::new(ui),
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell) {
        match &mut self.current_page {
            Page::Login => {
                self.login_page.perform(ui, &mut self.content);
                if self.login_page.change_state {
                    let temp = self.login_page.clone();
                    self.change_state_from_login(temp);
                }
            },
            Page::CharacterSelection => {
                self.character_selection_page.perform(ui, &mut self.content);
            },
        }
    }

    fn change_state_from_login(&mut self, x: Login) {
        match x.action {
            Some(login::Action::Success) => {
                self.current_page = Page::CharacterSelection
            },
            None => {},
        }
    }
}
