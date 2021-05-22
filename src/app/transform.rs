use crate::linalg::Vec2;
use crate::app::imgui::ImDraw;

#[derive(Copy, Clone, Debug, Default, ImDraw)]
pub struct Transform {
    pub pos: Vec2,
    //pub scale: Vec2,
    pub rot: f32,
    pub layer: i32,
}

impl Transform {
    pub fn from_pos(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2 { x, y },
            rot: 0.0,
            layer: 0,
        }
    }
}
