mod content;
mod login;
mod character_selection;
mod town;
mod tex;

use self::login::Login;
use self::character_selection::CharacterSelection;
use self::town::Town;
use self::content::Content;
use self::tex::Tex;

pub struct State<'a, I: graphics::ImageSize> {
    current_page: Page,
    pub chat_receiver: Option<&'a std::sync::mpsc::Receiver<String>>,
    pub chat_sender: Option<&'a std::sync::mpsc::Sender<String>>,
    pub move_receiver: Option<&'a std::sync::mpsc::Receiver<String>>,
    pub move_sender: Option<&'a std::sync::mpsc::Sender<String>>,
    pub content: Content,
    pub tex: Tex<I>,

    character_selection_page: CharacterSelection,
    login_page: Login,
    town_page: Town,
}

enum Page {
    Login,
    CharacterSelection,
    Town,
}

impl<'a, I: graphics::ImageSize> State<'a, I> {
    pub fn new(ui: &mut conrod_core::Ui, player_tex: std::rc::Rc<I>) -> Self {
        Self {
            login_page: Login::new(ui),
            current_page: Page::Login,
            content: Content::new(),
            chat_receiver: None,
            chat_sender: None,
            move_receiver: None,
            move_sender: None,
            character_selection_page: CharacterSelection::new(ui),
            town_page: Town::new(ui),

            tex: Tex::new(player_tex.clone()),
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell, scene: &mut sprite::Scene<I>, event: &piston_window::Event) {
        self.manage_chat();
        self.manage_move(scene, ui);

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
                    self.change_state_from_character_selection_page(temp, scene);
                }
            },
            Page::Town => {
                self.town_page.perform(ui, &mut self.content, self.chat_sender, &event, self.move_sender)
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

    fn change_state_from_character_selection_page(&mut self, x: CharacterSelection, scene: &mut sprite::Scene<I>) {
        match x.action {
            Some(character_selection::Action::EnterGame) => {
                self.current_page = Page::Town;
                self.town_page.start(scene, &self.tex , &mut self.content)
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

    fn manage_move(&mut self, scene: &mut sprite::Scene<I>, ui: &mut conrod_core::UiCell) {
        match self.move_receiver {
            Some(rx) => {
                let received = rx.try_recv();
                match received {
                    Ok(msg) => self.town_page.manage_move(msg, scene, ui),
                    Err(_) => {}
                }
            },
            None => {},
        }
    }
}
