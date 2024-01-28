use crate::game::components::{animation::Animation, sprite_data::SpriteData};
use hecs::World;

pub struct AnimationController;

impl AnimationController {
    pub fn run(world: &World, delta_time: f32) {
        for (_, (animation,)) in world.query::<(&mut Animation,)>().iter() {
            if let Some(animation) = animation.animation.as_mut() {
                animation.animation.update(delta_time);
            }
        }

        for (_, (animation, sprite_data)) in world.query::<(&Animation, &mut SpriteData)>().iter() {
            if let Some(animation) = animation.animation.as_ref() {
                if let Some(frame) = animation.animation.current_frame() {
                    sprite_data.texture = format!("{}/{}", animation.id, frame).into();
                }
            }
        }
    }
}
