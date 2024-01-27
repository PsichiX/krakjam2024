use micro_games_kit::third_party::{vek::Rgba, vek::Vec2};
use std::borrow::Cow;

pub struct SpriteData {
    pub shader: Cow<'static, str>,
    pub texture: Cow<'static, str>,
    pub tint: Rgba<f32>,
    pub pivot: Vec2<f32>,
}
