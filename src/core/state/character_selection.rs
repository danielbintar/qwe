use crate::core::state::content::Content;

pub struct CharacterSelection {
    ids: Ids,
}

widget_ids! {
    struct Ids {
        canvas,
        title,
        description,
        flash,
    }
}

impl CharacterSelection {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
        }
    }

    pub fn perform(&mut self, ui: &mut conrod_core::UiCell, content: &mut Content) {
    	use conrod_core::{widget, Positionable, Sizeable, Widget};

        const MARGIN: conrod_core::Scalar = 30.0;
        const TITLE_SIZE: conrod_core::FontSize = 42;

        const TITLE: &'static str = "Immortal";
        widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(self.ids.canvas, ui);

        widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(self.ids.canvas).set(self.ids.title, ui);

        const DESCRIPTION: &'static str = "Game for eternity!";
        widget::Text::new(DESCRIPTION)
            .padded_w_of(self.ids.canvas, MARGIN)
            .down_from(self.ids.title, 50.0)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            .line_spacing(5.0)
            .set(self.ids.description, ui);

       let notice =
            match &content.current_user {
                Some(current_user) => format!("Hello, {}", current_user.username),
                None => String::from("Hello, guest"),
            };

        widget::Text::new(&notice[..])
                .padded_w_of(self.ids.canvas, MARGIN)
                .down_from(self.ids.title, 100.0)
                .align_middle_x_of(self.ids.canvas)
                .center_justify()
                .line_spacing(5.0)
                .set(self.ids.flash, ui);
    }
}
