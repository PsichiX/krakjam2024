use micro_games_kit::third_party::{vek::Rgba, vek::Vec2};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteData {
    pub shader: Cow<'static, str>,
    pub texture: Cow<'static, str>,
    pub tint: Rgba<f32>,
    pub pivot: Vec2<f32>,
    pub offset: Vec2<f32>,
    pub scale: Vec2<f32>,
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            shader: "image".into(),
            texture: Default::default(),
            tint: Rgba::white(),
            pivot: 0.5.into(),
            offset: Default::default(),
            scale: 1.0.into(),
        }
    }
}
