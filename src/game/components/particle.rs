use std::{borrow::Cow, ops::RangeInclusive};

use micro_games_kit::third_party::{
    rand::{thread_rng, Rng},
    spitfire_draw::particles::ParticleInstance,
    vek::{Rgba, Transform, Vec2},
};

pub struct Particle {
    pub texture: Cow<'static, str>,
    pub lifetime: f32,
    pub lifetime_max: f32,
    pub scale: f32,
}

impl Particle {
    pub fn generate_velocity(speed: RangeInclusive<f32>, angle_range: f32) -> Vec2<f32> {
        let mut rng = thread_rng();
        let speed = rng.gen_range(speed);
        let angle = rng.gen_range((-angle_range)..=angle_range);
        let (vx, vy) = angle.sin_cos();

        Vec2 { x: vx, y: -vy } * speed
    }

    pub fn new(
        texture: Cow<'static, str>,
        lifetime_max: RangeInclusive<f32>,
        scale: RangeInclusive<f32>,
    ) -> Self {
        let mut rng = thread_rng();
        let lifetime_max = rng.gen_range(lifetime_max);
        let scale = rng.gen_range(scale);

        Self {
            texture,
            lifetime: lifetime_max,
            lifetime_max,
            scale,
        }
    }

    pub fn emit(&self, transform: &Transform<f32, f32, f32>) -> Option<ParticleInstance> {
        let alpha = self.lifetime / self.lifetime_max;
        if alpha > 0.0 {
            Some(ParticleInstance {
                tint: Rgba {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: alpha,
                },
                transform: transform.clone(),
                size: Vec2 {
                    x: 60.0 * self.scale,
                    y: 60.0 * self.scale,
                },
                pivot: 0.5.into(),
                ..Default::default()
            })
        } else {
            None
        }
    }
}
