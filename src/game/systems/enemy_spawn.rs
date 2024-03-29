use crate::game::{
    components::{
        animation::Animation,
        collidable::Collidable,
        damage::{Damage, DamageLayer},
        effect::Effect,
        enemy::Enemy,
        health::Health,
        immobility::Immobility,
        player::Player,
        speed::Speed,
        sprite_data::SpriteData,
    },
    utils::space::SpaceObject,
};
use hecs::World;
use micro_games_kit::{
    animation::{FrameAnimation, NamedAnimation},
    third_party::{
        rand::{thread_rng, Rng},
        vek::{Transform, Vec2},
    },
};

pub struct EnemySpawn {
    pub distance_from_player: f32,
    pub interval_seconds: f32,
    pub count_limit: usize,
    time_accumulator: f32,
}

impl EnemySpawn {
    pub fn new(distance_from_player: f32, interval_seconds: f32, count_limit: usize) -> Self {
        Self {
            distance_from_player,
            interval_seconds,
            count_limit,
            time_accumulator: 0.0,
        }
    }

    pub fn run(&mut self, world: &mut World, delta_time: f32) {
        let count = world.query::<&Enemy>().iter().count();
        if count >= self.count_limit {
            return;
        }

        let query = world
            .query::<&Transform<f32, f32, f32>>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|(_, transform)| transform.position.xy());

        if let Some(player_position) = query {
            self.time_accumulator -= delta_time;
            if self.time_accumulator > 0.0 {
                return;
            }
            self.time_accumulator = self.interval_seconds;
            let angle = thread_rng().gen_range(-180.0f32..180.0f32).to_radians();
            let (y, x) = angle.sin_cos();
            let position = player_position + Vec2 { x, y } * self.distance_from_player;
            let effect = match thread_rng().gen_range(0..4) {
                0 => Effect::default(),
                1 => Effect {
                    fire: true,
                    ..Default::default()
                },
                2 => Effect {
                    water: true,
                    ..Default::default()
                },
                3 => Effect {
                    electricity: true,
                    ..Default::default()
                },
                _ => unreachable!(),
            };

            let kind = if thread_rng().gen_ratio(1, 3) {
                "truck"
            } else {
                "slime"
            };
            let _ = world.spawn((
                Enemy::default(),
                Animation {
                    animation: Some(NamedAnimation {
                        animation: FrameAnimation::new(0..1).fps(10.0).looping().playing(),
                        id: kind.to_owned(),
                    }),
                },
                Transform::<f32, f32, f32> {
                    position: position.into(),
                    ..Default::default()
                },
                Collidable {
                    space_object: Some(SpaceObject {
                        entity: None,
                        position,
                        collider_radius: 30.0,
                    }),
                },
                SpriteData {
                    texture: format!("{}/0", kind).into(),
                    ..Default::default()
                },
                effect,
                Health {
                    value: 50.0,
                    limit: 50.0,
                    layer: DamageLayer::Enemy,
                },
                Damage {
                    value: 1.0,
                    layer: DamageLayer::Enemy,
                },
                Speed::new(40.0..=100.0),
                Immobility { time_left: 0.0 },
            ));
        }
    }
}
