mod content;
mod login;
mod character_selection;
mod town;


use self::login::Login;
use self::character_selection::CharacterSelection;
use self::town::Town;
use self::content::Content;

pub struct State<'a> {
    current_page: Page,
    pub chat_receiver: Option<&'a std::sync::mpsc::Receiver<String>>,
    pub chat_sender: Option<&'a std::sync::mpsc::Sender<String>>,
    pub content: Content,

    character_selection_page: CharacterSelection,
    login_page: Login,
    town_page: Town,

    player_sprite_id: uuid::Uuid,
}

enum Page {
    Login,
    CharacterSelection,
    Town,
}

impl<'a> State<'a> {
    pub fn new(ui: &mut conrod_core::Ui, player_sprite_id: uuid::Uuid) -> Self {
        Self {
            login_page: Login::new(ui),
            current_page: Page::Login,
            content: Content::new(),
            chat_receiver: None,
            chat_sender: None,
            character_selection_page: CharacterSelection::new(ui),
            town_page: Town::new(ui),

            player_sprite_id,
        }
    }

    pub fn perform<I: graphics::ImageSize>(&mut self, ui: &mut conrod_core::UiCell, scene: &mut sprite::Scene<I>, event: &piston_window::Event) {
        self.manage_chat();

        match &mut self.current_page {
            Page::Login => {
                self.login_page.perform(ui, &mut self.content);
                if self.login_page.change_state {
                    let temp = self.login_page.clone();
                    self.change_state_from_login(temp);
                }
            },
            Page::CharacterSelection => {
                self.character_selection_page.perform(ui);
                if self.character_selection_page.change_state {
                    let temp = self.character_selection_page.clone();
                    self.change_state_from_character_selection_page(temp);
                }
            },
            Page::Town => {
                self.town_page.perform(ui, &mut self.content, self.chat_sender, scene, self.player_sprite_id, &event)
            }
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

    fn change_state_from_character_selection_page(&mut self, x: CharacterSelection) {
        match x.action {
            Some(character_selection::Action::EnterGame) => {
                self.current_page = Page::Town
            },
            None => {},
        }
    }

    fn manage_chat(&mut self) {
        match self.chat_receiver {
            Some(rx) => {
                let received = rx.try_recv();
                match received {
                    Ok(msg) => self.content.chat.push(msg),
                    Err(_) => {}
                }
            },
            None => {},
        }
    }
}
