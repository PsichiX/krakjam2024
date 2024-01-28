use crate::game::components::sprite_data::SpriteData;
use hecs::World;
use micro_games_kit::{
    context::GameContext,
    third_party::{
        spitfire_draw::{
            sprite::{Sprite, SpriteTexture},
            utils::{Drawable, ShaderRef, TextureRef},
        },
        vek::Transform,
    },
};

pub struct SpriteRenderer;

impl SpriteRenderer {
    pub fn run(world: &World, context: &mut GameContext) {
        for (_, (transform, sprite_data)) in world
            .query::<(&Transform<f32, f32, f32>, &SpriteData)>()
            .iter()
        {
            let mut transform = *transform;
            transform.position += sprite_data.offset;
            transform.scale *= sprite_data.scale;
            let sprite = Sprite::single(SpriteTexture::new(
                "u_image".into(),
                TextureRef::name(sprite_data.texture.clone()),
            ))
            .transform(transform)
            .shader(ShaderRef::name(sprite_data.shader.clone()))
            .pivot(sprite_data.pivot)
            .tint(sprite_data.tint);
            sprite.draw(context.draw, context.graphics);
        }
    }
}
