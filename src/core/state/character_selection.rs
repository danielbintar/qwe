use crate::core::state::content::Content;

pub struct CharacterSelection {
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

impl CharacterSelection {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell, content: &mut Content, chat_sender: Option<&std::sync::mpsc::Sender<String>>) {
        use conrod_core::{widget, Labelable, Positionable, Sizeable, Widget};

        const MARGIN: conrod_core::Scalar = 30.0;

        widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(self.ids.canvas, ui);

       let notice =
            match &content.current_user {
                Some(current_user) => format!("Hello, {}", current_user.username),
                None => String::from("Hello, guest"),
            };

        widget::Text::new(&notice[..])
                .padded_w_of(self.ids.canvas, MARGIN)
                .mid_top_of(self.ids.canvas)
                .align_middle_x_of(self.ids.canvas)
                .center_justify()
                .line_spacing(5.0)
                .set(self.ids.flash, ui);

        for event in widget::TextBox::new(&content.current_chat[..])
            .padded_w_of(self.ids.canvas, MARGIN)
            .down_from(self.ids.flash, 120.0)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .set(self.ids.current_chat, ui)
        {
            match event {
                conrod_core::widget::text_box::Event::Update(text) => content.current_chat = text,
                _ => println!("enter pressed")
            }
        }

        let side = 150.0;

        for _press in widget::Button::new()
            .label("SEND")
            .down_from(self.ids.current_chat, 150.0)
            .w_h(side, side)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.button, ui)
        {
            self.send_message(content, chat_sender)
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
