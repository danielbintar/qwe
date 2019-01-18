use std::collections::HashMap;

use crate::core::state::content::Content;
use crate::core::flash_message::FlashMessage;

#[derive(Clone)]
pub struct Login {
    username: String,
    password: String,
    notice: Option<FlashMessage>,
    ids: Ids,
    pub change_state: bool,
    pub action: Option<Action>,
}

#[derive(Clone)]
pub enum Action {
    Success
}

widget_ids! {
    #[derive(Clone)]
    struct Ids {
        canvas,
        title,
        description,
        username,
        password,
        button,
        flash,
    }
}

impl Login {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Self {
            username: String::from("username"),
            password: String::from("password"),
            notice: None,
            ids: Ids::new(ui.widget_id_generator()),
            change_state: false,
            action: None
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell, content: &mut Content) {
        use conrod_core::{widget, Labelable, Positionable, Sizeable, Widget};

        const MARGIN: conrod_core::Scalar = 30.0;
        const TITLE_SIZE: conrod_core::FontSize = 42;

        const TITLE: &'static str = "Immortal";
        widget::Canvas::new().pad(MARGIN).set(self.ids.canvas, ui);

        widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(self.ids.canvas).set(self.ids.title, ui);

        const DESCRIPTION: &'static str = "Game for eternity!";
        widget::Text::new(DESCRIPTION)
            .padded_w_of(self.ids.canvas, MARGIN)
            .down_from(self.ids.title, 50.0)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .line_spacing(5.0)
            .set(self.ids.description, ui);

        for event in widget::TextBox::new(&self.username[..])
            .padded_w_of(self.ids.canvas, MARGIN)
            .down_from(self.ids.title, 100.0)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .set(self.ids.username, ui)
        {
            match event {
                conrod_core::widget::text_box::Event::Update(text) => self.username = text,
                _ => println!("enter pressed")
            }
        }

        for event in widget::TextBox::new(&self.password[..])
            .padded_w_of(self.ids.canvas, MARGIN)
            .down_from(self.ids.title, 120.0)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .set(self.ids.password, ui)
        {
            match event {
                conrod_core::widget::text_box::Event::Update(text) => self.password = text,
                _ => println!("enter pressed")
            }
        }

        let side = 150.0;

        for _press in widget::Button::new()
            .label("LOGIN")
            .down_from(self.ids.title, 150.0)
            .w_h(side, side)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.button, ui)
        {
            self.login_request(content)
        }


        let notice =
            match &content.current_user {
                Some(current_user) => format!("Hello, {}", current_user.username),
                None => String::from("Hello, guest"),
            };

        widget::Text::new(&notice[..])
                .padded_w_of(self.ids.canvas, MARGIN)
                .down_from(self.ids.button, 100.0)
                .align_middle_x_of(self.ids.canvas)
                .center_justify()
                .line_spacing(5.0)
                .set(self.ids.flash, ui);
    }

    fn login_request(&mut self, content: &mut Content) {
        let mut map = HashMap::new();
        map.insert("username", &self.username[..]);
        map.insert("password", &self.password[..]);

        let mut resp = reqwest::Client::new()
            .post("http://localhost:3333/users/sign_in")
            .json(&map)
            .send().unwrap();

        if resp.status().is_success() {
            content.current_user = Some(resp.json().unwrap());
            self.change_state = true;
            self.action = Some(Action::Success);
        } else if resp.status().is_server_error() {

        } else {

        }
    }
}
