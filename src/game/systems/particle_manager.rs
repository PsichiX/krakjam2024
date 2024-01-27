use std::ops::RangeInclusive;

use hecs::World;
use micro_games_kit::{
    context::GameContext,
    third_party::{
        rand::{thread_rng, Rng},
        spitfire_draw::{
            particles::{
                ParticleEmitter, ParticleInstance, ParticleSystem, ParticleSystemProcessor,
            },
            sprite::SpriteTexture,
            utils::{Drawable, ShaderRef, TextureRef},
        },
        spitfire_glow::renderer::GlowTextureFiltering,
        vek::{Rgba, Transform, Vec2},
    },
};

use crate::game::components::particle_generator::ParticleGenerator;

pub struct GenericParticlesProcessor {}

impl ParticleSystemProcessor<GenericParticleData, f32> for GenericParticlesProcessor {
    fn process(delta_time: &f32, mut data: GenericParticleData) -> Option<GenericParticleData> {
        data.lifetime -= *delta_time;
        if data.lifetime >= 0.0 {
            data.position += data.velocity * *delta_time;
            data.velocity.x *= data.stabilization;
            Some(data)
        } else {
            None
        }
    }

    fn emit(_: &f32, data: &GenericParticleData) -> Option<ParticleInstance> {
        let alpha = data.lifetime / data.lifetime_max;
        if alpha > 0.0 {
            Some(ParticleInstance {
                tint: Rgba {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: alpha,
                },
                transform: Transform {
                    position: data.position.into(),
                    ..Default::default()
                },
                size: Vec2 { x: 60.0, y: 60.0 },
                pivot: 0.5.into(),
                ..Default::default()
            })
        } else {
            None
        }
    }
}

pub struct GenericParticleData {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    stabilization: f32,
    lifetime: f32,
    lifetime_max: f32,
}

impl GenericParticleData {
    pub fn new(
        position: Vec2<f32>,
        angle_range: f32,
        speed: RangeInclusive<f32>,
        stabilization: RangeInclusive<f32>,
        lifetime_max: RangeInclusive<f32>,
    ) -> Self {
        let mut rng = thread_rng();
        let angle = rng.gen_range((-angle_range)..=angle_range);
        let speed = rng.gen_range(speed);
        let stabilization = rng.gen_range(stabilization);
        let lifetime_max = rng.gen_range(lifetime_max);
        let (vx, vy) = angle.sin_cos();
        Self {
            position,
            velocity: Vec2 { x: vx, y: -vy } * speed,
            stabilization,
            lifetime: lifetime_max,
            lifetime_max,
        }
    }
}

pub struct ParticleManager {
    pub system: ParticleSystem<GenericParticlesProcessor, GenericParticleData, f32>,
}

impl ParticleManager {
    pub fn process(&mut self, world: &World, delta_time: f32) {
        self.system.config = delta_time;
        self.system.process();

        for (_, (generator, transform)) in world
            .query::<(&mut ParticleGenerator, &Transform<f32, f32, f32>)>()
            .iter()
        {
            generator.emmission_accumulator += delta_time * 5.0;
            while generator.emmission_accumulator > 0.0 {
                generator.emmission_accumulator -= 1.0;
                self.system.push(GenericParticleData::new(
                    transform.position.xy() + Vec2::new(0.0, -22.0),
                    60.0f32.to_radians(),
                    10.0..=20.0,
                    0.85..=0.95,
                    2.0..=3.0,
                ))
            }
        }
    }

    pub fn draw(&self, world: &World, context: &mut GameContext) {
        for (_, (generator,)) in world.query::<(&ParticleGenerator,)>().iter() {
            ParticleEmitter::single(SpriteTexture {
                sampler: "u_image".into(),
                texture: TextureRef::name(generator.texture.clone()),
                filtering: GlowTextureFiltering::Linear,
            })
            .shader(ShaderRef::name("image"))
            .emit(self.system.emit())
            .draw(context.draw, context.graphics);
        }
    }
}
