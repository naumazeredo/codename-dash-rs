use crate::linalg::{Vec2, Vec2i};
use super::{
    Program,
    color::Color,
    texture::{Texture, TextureFlip},
};

#[derive(Copy, Clone, Debug)]
pub enum Command {
    DrawSprite {
        texture_flip: TextureFlip,
        uvs: (Vec2i, Vec2i),
        pivot: Vec2,
        size: Vec2,
    },
}

#[derive(Copy, Clone, Debug)]
pub struct DrawCommand {
    pub program: Program,
    pub texture: Texture,
    pub color: Color,

    pub pos: Vec2,
    pub rot: f32,
    pub layer: i32,

    pub cmd: Command,
}
