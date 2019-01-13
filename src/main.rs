extern crate find_folder;
extern crate piston_window;
extern crate conrod_piston;

use self::piston_window::{PistonWindow, UpdateEvent, Window, WindowSettings};
use self::piston_window::{G2d, G2dTexture, TextureSettings};
use self::piston_window::OpenGL;
use self::piston_window::texture::UpdateTexture;

use qwe::core::flash_message::FlashMessage;
use qwe::core::object::user::User;

use std::collections::HashMap;

#[macro_use] extern crate conrod_core;
extern crate rand;

extern crate reqwest;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

pub struct LoginForm {
    username: String,
    password: String,
    notice: Option<FlashMessage>,
}


impl LoginForm {
    pub fn new(username: String, password: String) -> Self {
        LoginForm {
            username,
            password,
            notice: None,
        }
    }
}

pub fn login_request(login_form: &mut LoginForm) {
    let mut map = HashMap::new();
    map.insert("username", &login_form.username[..]);
    map.insert("password", &login_form.password[..]);

    let mut resp = reqwest::Client::new()
        .post("http://localhost:3333/users/sign_in")
        .json(&map)
        .send().unwrap();

    if resp.status().is_success() {
        let user: User = resp.json().unwrap();
        let notice = format!("Hello, {}", user.username);
        login_form.notice = Some(FlashMessage::new(notice));
    } else if resp.status().is_server_error() {
        login_form.notice = Some(FlashMessage::new(String::from("server error!")));
    } else {
        let notice = format!("Something else happened. Status: {:?}", resp.status());
        login_form.notice = Some(FlashMessage::new(notice))
    }
}

pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        title,
        description,
        username,
        password,
        button,
        flash,
    }
}

pub fn gui(ui: &mut conrod_core::UiCell, ids: &Ids, login_form: &mut LoginForm) {
    use conrod_core::{widget, Labelable, Positionable, Sizeable, Widget};

    const MARGIN: conrod_core::Scalar = 30.0;
    const TITLE_SIZE: conrod_core::FontSize = 42;

    const TITLE: &'static str = "Immortal";
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    const DESCRIPTION: &'static str = "Game for eternity!";
    widget::Text::new(DESCRIPTION)
        .padded_w_of(ids.canvas, MARGIN)
        .down_from(ids.title, 50.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.description, ui);

    for event in widget::TextBox::new(&login_form.username[..])
        .padded_w_of(ids.canvas, MARGIN)
        .down_from(ids.title, 100.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .set(ids.username, ui)
    {
        match event {
            conrod_core::widget::text_box::Event::Update(text) => login_form.username = text,
            _ => println!("enter pressed")
        }
    }

    for event in widget::TextBox::new(&login_form.password[..])
        .padded_w_of(ids.canvas, MARGIN)
        .down_from(ids.title, 120.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .set(ids.password, ui)
    {
        match event {
            conrod_core::widget::text_box::Event::Update(text) => login_form.password = text,
            _ => println!("enter pressed")
        }
    }

    let side = 150.0;

    for _press in widget::Button::new()
        .label("LOGIN")
        .down_from(ids.title, 150.0)
        .w_h(side, side)
        .align_middle_x_of(ids.canvas)
        .set(ids.button, ui)
    {
        login_request(login_form)
    }

    if let Some(notice) = &login_form.notice {
        widget::Text::new(&notice.text[..])
            .padded_w_of(ids.canvas, MARGIN)
            .down_from(ids.button, 100.0)
            .align_middle_x_of(ids.canvas)
            .center_justify()
            .line_spacing(5.0)
            .set(ids.flash, ui);
    }
}

pub fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;

    let mut window: PistonWindow =
        WindowSettings::new("qwe", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .samples(4)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(theme())
        .build();

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/FiraSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(WIDTH, HEIGHT)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings).unwrap();
        (cache, texture)
    };

    let ids = Ids::new(ui.widget_id_generator());

    let image_map = conrod_core::image::Map::new();

    let mut login_form = LoginForm::new(String::from("username"), String::from("password"));

    while let Some(event) = window.next() {

        let size = window.size();
        let (win_w, win_h) = (size.width as conrod_core::Scalar, size.height as conrod_core::Scalar);
        if let Some(e) = conrod_piston::event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            gui(&mut ui, &ids, &mut login_form);
        });

        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {

                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod_core::text::rt::Rect<u32>,
                                           data: &[u8]|
                    {
                        let offset = [rect.min.x, rect.min.y];
                        let size = [rect.width(), rect.height()];
                        let format = piston_window::texture::Format::Rgba8;
                        let encoder = &mut graphics.encoder;
                        text_vertex_data.clear();
                        text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                        UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
                            .expect("failed to update texture")
                    };

                fn texture_from_image<T>(img: &T) -> &T { img }

                conrod_piston::draw::primitives(primitives,
                                                context,
                                                graphics,
                                                &mut text_texture_cache,
                                                &mut glyph_cache,
                                                &image_map,
                                                cache_queued_glyphs,
                                                texture_from_image);
            }
        });
    }
}
