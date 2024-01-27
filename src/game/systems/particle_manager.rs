use hecs::{Entity, World};
use micro_games_kit::{
    context::GameContext,
    third_party::{
        spitfire_draw::{
            particles::ParticleEmitter,
            sprite::SpriteTexture,
            utils::{Drawable, ShaderRef, TextureRef},
        },
        spitfire_glow::renderer::GlowTextureFiltering,
        vek::{Transform, Vec2},
    },
};

use crate::game::components::{
    particle::Particle, particle_generator::ParticleGenerator, projectile::Projectile, spell::Spell,
};
pub struct ParticleManager {}

impl ParticleManager {
    pub fn process(&mut self, world: &mut World, delta_time: f32) {
        let mut particles = Vec::<Particle>::new();

        for (_, (generator, transform, projectile, spell)) in world
            .query::<(
                &mut ParticleGenerator,
                &Transform<f32, f32, f32>,
                Option<&Projectile>,
                Option<&Spell>,
            )>()
            .iter()
        {
            generator.emmission_accumulator += delta_time;

            let velocity = match projectile {
                None => Vec2::<f32>::default(),
                Some(p) => p.velocity,
            };

            let scale_offset = match spell {
                None => 0.0,
                Some(spell) => spell.size.scale_offset(),
            };

            while generator.emmission_accumulator > generator.emmission_time {
                generator.emmission_accumulator = 0.0;

                for _ in 0..generator.batch_size {
                    particles.push(Particle::new(
                        generator.texture.clone(),
                        transform.position.xy() + Vec2::new(0.0, -22.0),
                        velocity,
                        180.0f32.to_radians(),
                        100.0..=200.0,
                        0.1..=0.5,
                        (0.8 + scale_offset)..=(1.5 + scale_offset),
                    ));
                }
            }
        }

        for particle in particles {
            world.spawn((particle,));
        }

        let mut entities_to_remove = Vec::<Entity>::new();

        for (entity, (data,)) in world.query::<(&mut Particle,)>().iter() {
            data.lifetime -= delta_time;

            if data.lifetime >= 0.0 {
                data.position += data.velocity * delta_time;
            } else {
                entities_to_remove.push(entity);
            }
        }

        for entity in entities_to_remove {
            let _ = world.despawn(entity);
        }
    }

    pub fn draw(&self, world: &World, context: &mut GameContext) {
        for (_, (particle,)) in world.query::<(&Particle,)>().iter() {
            ParticleEmitter::single(SpriteTexture {
                sampler: "u_image".into(),
                texture: TextureRef::name(particle.texture.clone()),
                filtering: GlowTextureFiltering::Linear,
            })
            .shader(ShaderRef::name("image"))
            .emit(particle.emit())
            .draw(context.draw, context.graphics);
        }
    }
}
