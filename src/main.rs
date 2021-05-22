// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(dead_code))]

//#![feature(option_expect_none)]

#[macro_use] extern crate bitflags;
extern crate imgui;
extern crate imgui_opengl_renderer;

#[macro_use] mod app;
//mod entities;
mod linalg;

use app::*;
use linalg::*;

fn main() {
    App::<State>::new().run();
}

#[derive(ImDraw)]
pub struct State {
    pub input_mapping: InputMapping,
    pub font: Font,
    pub texture: Texture,
    pub sprites: Sprites,
}

#[derive(ImDraw)]
pub struct Sprites {
    pub target_sprite: Sprite,
    pub short_note_sprite: Sprite,
    pub long_note_sprites: (Sprite, Sprite, Sprite),
    pub rhythm_line_sprite: Sprite,
}

impl GameState for State {
    fn new(app: &mut App<'_, Self>) -> Self {
        // Fonts
        let font = app.bake_font("assets/fonts/Monocons.ttf").unwrap();

        // Animation
        let texture = app.get_texture("assets/gfx/gfx.png");

        let build_sprite = |x, y, w, h| {
            Sprite {
                texture,
                texture_flip: TextureFlip::NO,
                uvs: (Vec2i { x, y }, Vec2i { x: w + x, y: h + y }),
                pivot: Vec2 { x: w as f32 / 2., y: h as f32 / 2. },
                size: Vec2 { x: w as f32, y: h as f32 },
            }
        };

        let target_sprite = build_sprite(0, 0, 32, 32);
        let short_note_sprite = build_sprite(32, 0, 16, 16);
        let long_note_sprites = (
            build_sprite(48, 0, 16, 16),
            build_sprite(64, 0, 16, 16),
            build_sprite(80, 0, 16, 16),
        );
        let rhythm_line_sprite = build_sprite(96, 0, 16, 16);

        // input
        let mut input_mapping = InputMapping::new();

        {
            let mut button = Button::new();
            button.add_key(sdl2::keyboard::Scancode::W);
            button.add_controller_button(0, sdl2::controller::Button::DPadUp);
            button.add_controller_axis(
                0,
                sdl2::controller::Axis::LeftY,
                ControllerAxisThreshold::lesser_than(-0.5)
            );

            input_mapping.add_button_mapping("UP".to_string(), button);
        }

        {
            let mut button = Button::new();
            button.add_key(sdl2::keyboard::Scancode::S);
            button.add_controller_button(0, sdl2::controller::Button::DPadDown);
            button.add_controller_axis(
                0,
                sdl2::controller::Axis::LeftY,
                ControllerAxisThreshold::greater_than(0.5)
            );

            input_mapping.add_button_mapping("DOWN".to_string(), button);
        }

        {
            let mut button = Button::new();
            button.add_key(sdl2::keyboard::Scancode::D);
            button.add_controller_button(0, sdl2::controller::Button::DPadRight);
            button.add_controller_axis(
                0,
                sdl2::controller::Axis::LeftX,
                ControllerAxisThreshold::greater_than(0.5)
            );

            input_mapping.add_button_mapping("RIGHT".to_string(), button);
        }

        {
            let mut button = Button::new();
            button.add_key(sdl2::keyboard::Scancode::A);
            button.add_controller_button(0, sdl2::controller::Button::DPadLeft);
            button.add_controller_axis(
                0,
                sdl2::controller::Axis::LeftX,
                ControllerAxisThreshold::lesser_than(-0.5)
            );

            input_mapping.add_button_mapping("LEFT".to_string(), button);
        }

        Self {
            input_mapping,
            font,
            texture,
            sprites: Sprites {
                target_sprite,
                short_note_sprite,
                long_note_sprites,
                rhythm_line_sprite,
            }
        }
    }

    fn update(&mut self, app: &mut App<'_, Self>) {
        app.update_input_mapping(&mut self.input_mapping);
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        app.queue_draw_text(
            "Hello world",
            &self.font,
            &Transform {
                pos: Vec2 { x: 200., y: 200. },
                rot: 0.,
                layer: 0,
            },
            32.,
            WHITE
        );

        app.queue_draw_sprite(
            &Transform::from_pos(100.0, 100.0),
            &self.sprites.target_sprite,
            WHITE
        );

        app.render_queued();

        // @Refactor maybe this debug info really should be managed by the App. This way
        //           we don't have to explicitly call render_queued, which seems way cleaner.
        //           Maybe not, since we can add framebuffers and have more control of rendering here.
        app.render_debug(self, |ui, state| {
            state.imdraw("State", ui);
        });
    }

    fn handle_input(&mut self, app: &mut App<'_, Self>, event: &sdl2::event::Event) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        if app.handle_debug_event(&event) { return true; }

        match event {
            Event::KeyDown { scancode: Some(Scancode::F11), .. } => {
                use sdl2::video::FullscreenType;

                let window = &mut app.video_system.window;
                let new_fullscreen_state = match window.fullscreen_state() {
                    //FullscreenType::Off => FullscreenType::True,
                    //FullscreenType::True => FullscreenType::Desktop,
                    //FullscreenType::Desktop => FullscreenType::Off,

                    FullscreenType::Off => FullscreenType::Desktop,
                    _ => FullscreenType::Off,
                };

                window.set_fullscreen(new_fullscreen_state).unwrap();
            },

            _ => {}
        }

        false
    }
}
