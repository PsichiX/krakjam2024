use std::f32::consts::PI;

use crate::game::components::{enemy::Enemy, sprite_data::SpriteData};
use hecs::World;

pub struct EnemyJumpAnimation;

impl EnemyJumpAnimation {
    pub fn run(world: &World, delta_time: f32) {
        for (_, (enemy, sprite_data)) in world.query::<(&mut Enemy, &mut SpriteData)>().iter() {
            enemy.jump_phase -= delta_time;
            while enemy.jump_phase < 0.0 {
                enemy.jump_phase += 1.0;
            }
            let phase = (enemy.jump_phase * PI * 2.0).sin().abs();
            sprite_data.offset.y = -phase * 30.0;
            sprite_data.scale.x = 1.0 - phase * 0.25;
            sprite_data.scale.y = 1.0 + phase * 0.25;
        }
    }
}
