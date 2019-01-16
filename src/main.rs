extern crate find_folder;
extern crate piston_window;
extern crate conrod_piston;

use self::piston_window::{PistonWindow, UpdateEvent, Window, WindowSettings};
use self::piston_window::{G2d, G2dTexture, TextureSettings};
use self::piston_window::OpenGL;
use self::piston_window::texture::UpdateTexture;

use qwe::core::state::State;

extern crate conrod_core;
extern crate rand;

extern crate reqwest;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate ws;
use ws::{connect, Handler, Sender, Handshake, Result, Message};
use std::thread;

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

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

struct Client {
    out: Sender,
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        Ok(())
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

    let mut state = State::new(&mut ui);


    thread::spawn( ||
        connect("ws://127.0.0.1:3333/chat", |out| Client { out: out } ).unwrap()
    );

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

    let image_map = conrod_core::image::Map::new();

    while let Some(event) = window.next() {

        let size = window.size();
        let (win_w, win_h) = (size.width as conrod_core::Scalar, size.height as conrod_core::Scalar);
        if let Some(e) = conrod_piston::event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            state.perform(&mut ui);
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
