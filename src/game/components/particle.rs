use std::{borrow::Cow, ops::RangeInclusive};

use micro_games_kit::third_party::{
    rand::{thread_rng, Rng},
    spitfire_draw::particles::ParticleInstance,
    vek::{Rgba, Transform, Vec2},
};

pub struct Particle {
    pub texture: Cow<'static, str>,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub lifetime: f32,
    pub lifetime_max: f32,
    pub scale: f32,
}

impl Particle {
    pub fn new(
        texture: Cow<'static, str>,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
        angle_range: f32,
        speed: RangeInclusive<f32>,
        lifetime_max: RangeInclusive<f32>,
        scale: RangeInclusive<f32>,
    ) -> Self {
        let mut rng = thread_rng();
        let angle = rng.gen_range((-angle_range)..=angle_range);
        let speed = rng.gen_range(speed);
        let lifetime_max = rng.gen_range(lifetime_max);
        let scale = rng.gen_range(scale);
        let (vx, vy) = angle.sin_cos();

        Self {
            texture,
            position,
            velocity: velocity + Vec2 { x: vx, y: -vy } * speed,
            lifetime: lifetime_max,
            lifetime_max,
            scale,
        }
    }

    pub fn emit(&self) -> Option<ParticleInstance> {
        let alpha = self.lifetime / self.lifetime_max;
        if alpha > 0.0 {
            Some(ParticleInstance {
                tint: Rgba {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: alpha,
                },
                transform: Transform {
                    position: self.position.into(),
                    ..Default::default()
                },
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
