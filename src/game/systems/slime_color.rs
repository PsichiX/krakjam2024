use crate::game::components::{effect::Effect, enemy::Enemy, sprite_data::SpriteData};
use hecs::World;

pub struct SlimeColor;

impl SlimeColor {
    pub fn run(world: &World) {
        for (_, (effect, sprite)) in world
            .query::<(&Effect, &mut SpriteData)>()
            .with::<&Enemy>()
            .iter()
        {
            sprite.tint = effect.slime_tint();
        }
    }
}
