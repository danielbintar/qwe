use crate::core::state::content::Content;
use piston_window::Event;
use piston::input::{Input, Button};

pub struct Town {
    ids: Ids,
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
        }
    }

    pub fn perform<I: graphics::ImageSize>(&mut self, ui: &mut conrod_core::UiCell, content: &mut Content, chat_sender: Option<&std::sync::mpsc::Sender<String>>, scene: &mut sprite::Scene<I>, player_sprite: uuid::Uuid, event: &Event) {
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

        let temp = scene.child_mut(player_sprite);
        match temp {
            Some(x) => { x.set_position(content.current_character.position.x, content.current_character.position.y) },
            None => {}
        }

        self.handle_input(ui, content, event);
    }

    fn handle_input(&mut self, ui: &mut conrod_core::UiCell, content: &mut Content, event: &Event) {
        if let Event::Input(input) = event {
            if let Input::Button(button_args) = input {
                if let Button::Keyboard(key) = button_args.button {
                    let mut moving = false;
                    match key {
                        input::keyboard::Key::Up => { content.current_character.position.y -= 10.0; moving = true },
                        input::keyboard::Key::Down => { content.current_character.position.y += 10.0; moving = true },
                        input::keyboard::Key::Right => { content.current_character.position.x += 10.0; moving = true },
                        input::keyboard::Key::Left => { content.current_character.position.x -= 10.0; moving = true },
                        _ => {}
                    }
                    if moving {
                        ui.needs_redraw()
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
