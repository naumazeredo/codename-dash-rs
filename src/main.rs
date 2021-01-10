// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate bitflags;
extern crate sdl2;
extern crate imgui;

mod app;
mod debug;
mod game_state;
mod imgui_sdl2;
mod linalg;
mod render;
mod time;

use app::App;
use render::*;
use linalg::*;
use imgui::*;
use game_state::GameState;

fn main() {
    App::new().run::<State>();
}

struct State {
    x: f32,
    y: f32,

    r: f32,

    px: f32,
    py: f32,

    w: f32,
    h: f32,

    c: render::Color,

    l: i32,

    texture: Texture,
}

impl GameState for State {
    fn new() -> Self {
        Self {
            x: 100.,
            y: 100.,
            r: 0.,
            px: 0.,
            py: 0.,
            w: 32.,
            h: 32.,
            c: render::WHITE,
            l: 0,
            texture: load_texture("assets/gfx/default.png"),
        }
    }

    // @XXX maybe change these functions to State methods and create a trait
    fn update(&mut self, _app: &mut App) {
    }

    fn render(&mut self, app: &mut App) {
        app.render.queue_draw_sprite(
            0 as Program,
            self.l,
            self.c,
            Vec2 { x: self.x, y: self.y }, Vec2 { x: self.w, y: self.h },
            self.r,
            Vec2 { x: self.px, y: self.py },
            self.texture,
            TextureFlip::NO,
            (Vec2i::new(), Vec2i::new())
        );

        // @Refactor maybe this debug info really should be managed by the App. This way
        //           we don't have to explicitly call render_queued, which seems way cleaner.
        //           Maybe not since we can add framebuffers and have more control of rendering here.

        app.render.render_queued_draws();

        // @TODO remove debug from App? Will I need to pass it in the callback functions also?
        //       Ideally we could just add it to the State, but we can't pass the state to its
        //       method if we do this :/
        app.debug.render(&app.window, &app.event_pump, self, |ui, state| {
            Drag::new(im_str!("x")).speed(0.1).build(&ui, &mut state.x);
            Drag::new(im_str!("y")).speed(0.1).build(&ui, &mut state.y);
            Drag::new(im_str!("r")).speed(0.1).build(&ui, &mut state.r);
            Drag::new(im_str!("px")).speed(0.1).build(&ui, &mut state.px);
            Drag::new(im_str!("py")).speed(0.1).build(&ui, &mut state.py);
            Drag::new(im_str!("w")).speed(0.1).build(&ui, &mut state.w);
            Drag::new(im_str!("h")).speed(0.1).build(&ui, &mut state.h);
            Drag::new(im_str!("l")).build(&ui, &mut state.l);

            let mut c: [f32; 4] = state.c.into();
            ColorEdit::new(im_str!("c"), &mut c).build(&ui);
            state.c = Color::from(c);
        });
    }
}
