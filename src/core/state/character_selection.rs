#[derive(Clone)]
pub struct CharacterSelection {
    ids: Ids,
    pub change_state: bool,
    pub action: Option<Action>,
}

#[derive(Clone)]
pub enum Action {
    EnterGame
}

widget_ids! {
    #[derive(Clone)]
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
            change_state: false,
            action: None
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell) {
        use conrod_core::{widget, Labelable, Positionable, Sizeable, Widget};

        const MARGIN: conrod_core::Scalar = 30.0;

        widget::Canvas::new().pad(MARGIN).set(self.ids.canvas, ui);

        let side = 150.0;

        for _press in widget::Button::new()
            .label("ENTER")
            .middle()
            .w_h(side, side)
            .set(self.ids.button, ui)
        {
            self.change_state = true;
            self.action = Some(Action::EnterGame);
        }
    }
}
