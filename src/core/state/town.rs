use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::core::state::content::Content;
use crate::core::object::user::UserPosition;
use piston_window::Event;
use piston::input::{Input, Button};

use sprite::Sprite;

pub struct Town {
    ids: Ids,
    players: Vec<UserPosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownResponse {
    id: u32,
    name: String,
    users: Vec<UserPosition>,
}

widget_ids! {
    struct Ids {
        canvas,
        flash,
        current_chat,
        button,
        chat,
    }
}

impl Town {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            players: Vec::new(),
        }
    }

    fn generate_players(&mut self) {
        let mut resp = reqwest::Client::new()
            .get("http://localhost:3333/towns/1?username=thisuser&password=password")
            .send().unwrap();

        if resp.status().is_success() {
            let response: TownResponse = resp.json().unwrap();
            self.players = response.users;
        } else if resp.status().is_server_error() {

        } else {

        }
    }

    pub fn manage_move<I: graphics::ImageSize>(&self, msg: String, scene: &mut sprite::Scene<I>, ui: &mut conrod_core::UiCell) {
        let user_position: UserPosition = serde_json::from_str(&msg).unwrap();

        let mut temp = None;
        for player in &self.players {
            if player.id == user_position.id {
                temp = player.sprite_id;
                break;
            }
        }
        match temp {
            Some(x) => {
                let tempp = scene.child_mut(x);
                match tempp {
                    Some(y) => {
                        y.set_position(user_position.get_x(), user_position.get_y());
                        ui.needs_redraw()
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    fn enter_request(&mut self, content: &mut Content) {
        let mut map = HashMap::new();
        let current_user = content.current_user.clone().unwrap();
        map.insert("username", &current_user.username[..]);
        map.insert("password", &current_user.password[..]);

        let mut resp = reqwest::Client::new()
            .post("http://localhost:3333/towns/1/enter")
            .json(&map)
            .send().unwrap();

        if resp.status().is_success() {
            let response: TownResponse = resp.json().unwrap();
            self.players = response.users;
        } else if resp.status().is_server_error() {

        } else {

        }
    }

    pub fn start<I: graphics::ImageSize>(&mut self, scene: &mut sprite::Scene<I>, player_tex: std::rc::Rc<I>, content: &mut Content) {
        self.generate_players();

        let mut found = false;
        for player in &mut self.players {
            let temp = &content.current_user;
            if let Some(current_user) = temp {
                if player.id == current_user.id {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            self.enter_request(content);
        }

        for player in &mut self.players {
            let mut sprite = Sprite::from_texture(player_tex.clone());
            sprite.set_position(player.get_x(), player.get_y());
            let id = scene.add_child(sprite);
            player.sprite_id = Some(id);

            let temp = &content.current_user;
            if let Some(current_user) = temp {
                if player.id == current_user.id {
                    content.current_character.position.x = player.get_real_x();
                    content.current_character.position.y = player.get_real_y();
                }
            }
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell, content: &mut Content, chat_sender: Option<&std::sync::mpsc::Sender<String>>, event: &Event, move_sender: Option<&std::sync::mpsc::Sender<String>>) {
        use conrod_core::{widget, Labelable, Positionable, Sizeable, Widget};

        const MARGIN: conrod_core::Scalar = 30.0;

        widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(self.ids.canvas, ui);

        let side = 150.0;

        for _press in widget::Button::new()
            .label("SEND")
            .middle()
            .w_h(side, side)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.button, ui)
        {
            self.send_message(content, chat_sender)
        }

        for event in widget::TextBox::new(&content.current_chat[..])
            .padded_w_of(self.ids.canvas, MARGIN)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .up_from(self.ids.button, 150.0)
            .set(self.ids.current_chat, ui)
        {
            match event {
                conrod_core::widget::text_box::Event::Update(text) => content.current_chat = text,
                _ => println!("enter pressed")
            }
        }

        let mut c = String::from("");

        for i in &content.chat {
            let new_chat = format!("\n{}", i);
            c.push_str(&new_chat[..])
        }

        widget::Text::new(&c[..])
            .padded_w_of(self.ids.canvas, MARGIN)
            .down_from(self.ids.button, 200.0)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .line_spacing(5.0)
            .set(self.ids.chat, ui);

        self.handle_input(content, event, move_sender);
    }

    fn handle_input(&mut self, content: &mut Content, event: &Event, move_sender: Option<&std::sync::mpsc::Sender<String>>) {
        let mut id = 0;
        match &content.current_user {
            Some(x) => {
                id = x.id;
            },
            None => {}
        }

        if let Event::Input(input) = event {
            if let Input::Button(button_args) = input {
                if let Button::Keyboard(key) = button_args.button {
                    let mut moving = false;
                    match key {
                        input::keyboard::Key::Up => { content.current_character.move_horizontal(1, false); moving = true },
                        input::keyboard::Key::Down => { content.current_character.move_horizontal(1, true); moving = true },
                        input::keyboard::Key::Right => { content.current_character.move_vertical(1, true); moving = true },
                        input::keyboard::Key::Left => { content.current_character.move_vertical(1, false); moving = true },
                        _ => {}
                    }

                    if moving {
                        match &move_sender {
                            Some(sender) => {
                                let m = UserPosition::new(id, content.current_character.position.x, content.current_character.position.y);
                                sender.send(serde_json::to_string(&m).unwrap()).unwrap();
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }

    fn send_message(&mut self, content: &mut Content, chat_sender: Option<&std::sync::mpsc::Sender<String>>) {
        let mut c = String::from("");
        match &content.current_user {
            Some(current_user) => {
                let d = format!("{}: ", current_user.username);
                c.push_str(&d[..])
            },
            None => {},
        };
        c.push_str(&content.current_chat[..]);
        match &chat_sender {
            Some(sender) => { sender.send(c).unwrap(); }
            None => {}
        }
    }
}
