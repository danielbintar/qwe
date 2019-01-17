mod login;
mod character_selection;
mod content;

use self::login::Login;
use self::character_selection::CharacterSelection;
use self::content::Content;

pub struct State<'a> {
    current_page: Page,
    pub chat_receiver: Option<&'a std::sync::mpsc::Receiver<String>>,
    pub chat_sender: Option<&'a std::sync::mpsc::Sender<String>>,
    pub content: Content,

    character_selection_page: CharacterSelection,
    login_page: Login,
}

enum Page {
    Login,
    CharacterSelection,
}

impl<'a> State<'a> {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Self {
            login_page: Login::new(ui),
            current_page: Page::Login,
            content: Content::new(),
            chat_receiver: None,
            chat_sender: None,
            character_selection_page: CharacterSelection::new(ui),
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell) {
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
                self.character_selection_page.perform(ui, &mut self.content, self.chat_sender);
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
