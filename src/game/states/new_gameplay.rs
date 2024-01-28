use crate::game::{
    components::{
        animation::Animation, collidable::Collidable, damage::Damage, effect::Effect,
        health::Health, ignore_entity::IgnoreEntity, immobility::Immobility,
        particle_generator::ParticleGenerator, sprite_data::SpriteData,
    },
    systems::{
        animation_controller::AnimationController, collision_detector::CollisionDetector,
        damage_dealer::DamageDealer, death::Death, effects_reactions::EffectsReactions,
        enemy_controller::EnemyController, enemy_spawn::EnemySpawn,
        immobility_controller::ImmobilityController, particle_manager::ParticleManager,
        player_controller::PlayerCastAction, slime_color::SlimeColor,
        spell_controller::SpellController,
    },
    ui::{health_bar::health_bar, world_to_screen_content_layout},
    utils::magic::spell_tag::{
        SpellTag, SpellTagDirection, SpellTagDuration, SpellTagEffect, SpellTagShape, SpellTagSize,
        SpellTagSpeed, SpellTagTrajectory,
    },
};
use crate::game::{
    components::{player::Player, projectile::Projectile},
    systems::{
        player_controller::PlayerController, projectile_controller::ProjectileController,
        sprite_renderer::SpriteRenderer,
    },
    utils::{magic::database::WordToSpellTagDatabase, space::SpaceObject},
};
use hecs::{Entity, World};
use micro_games_kit::{
    context::GameContext,
    game::{GameState, GameStateChange},
    third_party::{
        raui_core::layout::CoordsMappingScaling,
        raui_immediate_widgets::core::{
            containers::content_box, text_box, ContentBoxItemLayout, Rect, TextBoxFont,
            TextBoxHorizontalAlign, TextBoxProps, TextBoxVerticalAlign,
        },
        spitfire_draw::{
            sprite::{Sprite, SpriteTexture},
            utils::{Drawable, TextureRef},
        },
        spitfire_glow::{graphics::CameraScaling, renderer::GlowTextureFiltering},
        spitfire_input::{InputActionRef, InputConsume, InputMapping, VirtualAction},
        typid::ID,
        vek::{Rgba, Transform, Vec2},
        windowing::event::VirtualKeyCode,
    },
};

pub struct NewGameplay {
    map: [Sprite; 4],
    exit: InputActionRef,
    exit_handle: Option<ID<InputMapping>>,
    // music_forest: StaticSoundHandle,
    // music_battle: StaticSoundHandle,
    world: World,
    player_controller: PlayerController,
    enemy_spawn: EnemySpawn,
    particle_manager: ParticleManager,
    word_to_spell_tag_database: WordToSpellTagDatabase,
}

impl Default for NewGameplay {
    fn default() -> Self {
        // let mut audio = Audio::write();
        // let mut audio = audio.write().unwrap();

        // let mut music_forest = audio.play("forest").unwrap();
        // let _ = music_forest.set_volume(0.0, Default::default());
        // let _ = music_forest.set_loop_region(..);

        // let mut music_battle = audio.play("battle").unwrap();
        // let _ = music_battle.set_volume(0.0, Default::default());
        // let _ = music_battle.set_loop_region(..);

        Self {
            map: [
                Sprite::single(SpriteTexture {
                    sampler: "u_image".into(),
                    texture: TextureRef::name("map/tl"),
                    filtering: GlowTextureFiltering::Linear,
                })
                .pivot((1.0, 1.0).into()),
                Sprite::single(SpriteTexture {
                    sampler: "u_image".into(),
                    texture: TextureRef::name("map/tr"),
                    filtering: GlowTextureFiltering::Linear,
                })
                .pivot((0.0, 1.0).into()),
                Sprite::single(SpriteTexture {
                    sampler: "u_image".into(),
                    texture: TextureRef::name("map/bl"),
                    filtering: GlowTextureFiltering::Linear,
                })
                .pivot((1.0, 0.0).into()),
                Sprite::single(SpriteTexture {
                    sampler: "u_image".into(),
                    texture: TextureRef::name("map/br"),
                    filtering: GlowTextureFiltering::Linear,
                })
                .pivot((0.0, 0.0).into()),
            ],
            exit: Default::default(),
            exit_handle: None,
            world: World::new(),
            player_controller: PlayerController::default(),
            enemy_spawn: EnemySpawn::new(1000.0, 3.0, 30),
            particle_manager: ParticleManager {},
            word_to_spell_tag_database: WordToSpellTagDatabase::default()
                // Fire
                .with("fire", SpellTag::Effect(SpellTagEffect::Fire))
                .with("burn", SpellTag::Effect(SpellTagEffect::Fire))
                .with("heat", SpellTag::Effect(SpellTagEffect::Fire))
                .with("hot", SpellTag::Effect(SpellTagEffect::Fire))
                .with("meteor", SpellTag::Effect(SpellTagEffect::Fire))
                .with("fuck", SpellTag::Effect(SpellTagEffect::Fire))
                // Water
                .with("wet", SpellTag::Effect(SpellTagEffect::Water))
                .with("aqua", SpellTag::Effect(SpellTagEffect::Water))
                .with("h2o", SpellTag::Effect(SpellTagEffect::Water))
                .with("water", SpellTag::Effect(SpellTagEffect::Water))
                .with("fluid", SpellTag::Effect(SpellTagEffect::Water))
                .with("sprinkle", SpellTag::Effect(SpellTagEffect::Water))
                .with("drink", SpellTag::Effect(SpellTagEffect::Water))
                .with("cool", SpellTag::Effect(SpellTagEffect::Water))
                // Electricity
                .with("zap", SpellTag::Effect(SpellTagEffect::Electric))
                .with("power", SpellTag::Effect(SpellTagEffect::Electric))
                .with("tingly", SpellTag::Effect(SpellTagEffect::Electric))
                .with("charged", SpellTag::Effect(SpellTagEffect::Electric))
                .with("electro", SpellTag::Effect(SpellTagEffect::Electric))
                .with("electric", SpellTag::Effect(SpellTagEffect::Electric))
                .with("thunder", SpellTag::Effect(SpellTagEffect::Electric))
                // Large
                .with("big", SpellTag::Size(SpellTagSize::Large))
                .with("large", SpellTag::Size(SpellTagSize::Large))
                .with("enormous", SpellTag::Size(SpellTagSize::Large))
                .with("chungus", SpellTag::Size(SpellTagSize::Large))
                .with("chonker", SpellTag::Size(SpellTagSize::Large))
                .with("chonk", SpellTag::Size(SpellTagSize::Large))
                .with("meteor", SpellTag::Size(SpellTagSize::Large))
                .with("up", SpellTag::Size(SpellTagSize::Large))
                // Medium
                .with("fine", SpellTag::Size(SpellTagSize::Medium))
                .with("basic", SpellTag::Size(SpellTagSize::Medium))
                .with("boring", SpellTag::Size(SpellTagSize::Medium))
                // Small
                .with("tiny", SpellTag::Size(SpellTagSize::Small))
                .with("cute", SpellTag::Size(SpellTagSize::Small))
                .with("smol", SpellTag::Size(SpellTagSize::Small))
                .with("itsy", SpellTag::Size(SpellTagSize::Small))
                .with("little", SpellTag::Size(SpellTagSize::Small))
                .with("small", SpellTag::Size(SpellTagSize::Small))
                .with("small", SpellTag::Size(SpellTagSize::Small))
                .with("joke", SpellTag::Size(SpellTagSize::Small))
                // Sinus
                .with("sinus", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("nice", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                // Circle
                .with("circle", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("joke", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("fuck", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                // Slow
                .with("slow", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("turtle", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("boring", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("faster", SpellTag::Speed(SpellTagSpeed::Slow))
                // Medium
                .with("regular", SpellTag::Speed(SpellTagSpeed::Medium))
                // Fast
                .with("fast", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("joke", SpellTag::Speed(SpellTagSpeed::Fast))
                // Forward
                .with("forth", SpellTag::Direction(SpellTagDirection::Forward))
                // Backward
                .with("back", SpellTag::Direction(SpellTagDirection::Backward))
                .with("ass", SpellTag::Direction(SpellTagDirection::Backward))
                // Down
                .with("down", SpellTag::Direction(SpellTagDirection::Down))
                .with("stop", SpellTag::Direction(SpellTagDirection::Down))
                .with("me", SpellTag::Direction(SpellTagDirection::Down))
                .with("self", SpellTag::Direction(SpellTagDirection::Down))
                // Point
                .with("ball", SpellTag::Shape(SpellTagShape::Point))
                .with("basic", SpellTag::Shape(SpellTagShape::Point))
                .with("boring", SpellTag::Shape(SpellTagShape::Point))
                .with("meteor", SpellTag::Shape(SpellTagShape::Point))
                .with("joke", SpellTag::Shape(SpellTagShape::Point))
                // Wall
                .with("wall", SpellTag::Shape(SpellTagShape::Wall))
                .with("brick", SpellTag::Shape(SpellTagShape::Wall))
                .with("block", SpellTag::Shape(SpellTagShape::Wall))
                .with("rectangle", SpellTag::Shape(SpellTagShape::Wall))
                .with("square", SpellTag::Shape(SpellTagShape::Wall))
                .with("stuck", SpellTag::Shape(SpellTagShape::Wall))
                // Triangle
                .with("triangle", SpellTag::Shape(SpellTagShape::Triangle))
                .with("triforce", SpellTag::Shape(SpellTagShape::Triangle))
                // Quick
                .with("quick", SpellTag::Duration(SpellTagDuration::Quick))
                // Medium
                .with("moment", SpellTag::Duration(SpellTagDuration::Medium))
                .with("joke", SpellTag::Duration(SpellTagDuration::Medium))
                // Long
                .with("fuck", SpellTag::Duration(SpellTagDuration::Long))
                .with("long", SpellTag::Duration(SpellTagDuration::Long)),
        }
    }
}

impl GameState for NewGameplay {
    fn enter(&mut self, context: GameContext) {
        context.graphics.color = [0.0, 0.3, 0.0, 1.0];
        context.graphics.main_camera.screen_alignment = 0.5.into();
        context.graphics.main_camera.scaling = CameraScaling::FitVertical(800.0);
        context.gui.coords_map_scaling = CoordsMappingScaling::FitVertical(1024.0);

        self.exit_handle = Some(context.input.push_mapping(
            InputMapping::default().consume(InputConsume::Hit).action(
                VirtualAction::KeyButton(VirtualKeyCode::Escape),
                self.exit.clone(),
            ),
        ));

        self.player_controller.init(context.input);

        self.world.spawn((
            Player {
                current_effect_particle_accumulator: 0.0,
                current_effect_particle_time: 1.0,
            },
            Animation { animation: None },
            Transform::<f32, f32, f32>::default(),
            Collidable {
                space_object: Some(SpaceObject {
                    entity: None,
                    position: Vec2::default(),
                    collider_radius: 40.0,
                }),
            },
            Health { value: 100.0 },
            Effect {
                electricity: false,
                fire: false,
                water: false,
            },
            SpriteData {
                texture: "player/idle/0".into(),
                shader: "image".into(),
                pivot: 0.5.into(),
                tint: Rgba::white(),
            },
            Immobility { time_left: 0.0 },
        ));
    }

    fn exit(&mut self, context: GameContext) {
        if let Some(id) = self.exit_handle {
            context.input.remove_mapping(id);
            self.exit_handle = None;
        }

        // let _ = self.music_forest.stop(Default::default());
        // let _ = self.music_battle.stop(Default::default());
    }

    fn fixed_update(&mut self, mut context: GameContext, delta_time: f32) {
        if self.exit.get().is_down() {
            *context.state_change = GameStateChange::Pop;
        }

        self.enemy_spawn.run(&mut self.world, delta_time);
        self.player_controller.run(
            &mut self.world,
            &mut context,
            delta_time,
            &self.word_to_spell_tag_database,
        );
        EnemyController::run(&mut self.world, delta_time);
        AnimationController::run(&self.world, delta_time);
        ProjectileController::run(&self.world, delta_time);
        CollisionDetector::run(&self.world);
        EffectsReactions::run(&mut self.world);
        SpellController::run(&mut self.world);
        DamageDealer::run(&self.world);
        self.particle_manager.process(&mut self.world, delta_time);
        SlimeColor::run(&self.world);
        ImmobilityController::run(&self.world, delta_time);

        // always keep death last in the frame to run!
        Death::run(&mut self.world);

        if self.world.query::<&Player>().iter().next().is_none() {
            *context.state_change = GameStateChange::Swap(Box::new(NewGameplay::default()));
        }

        // self.update_ambient_music();
    }

    fn draw(&mut self, mut context: GameContext) {
        for sprite in &self.map {
            sprite.draw(context.draw, context.graphics);
        }

        SpriteRenderer::run(&self.world, &mut context);
        self.particle_manager.draw(&self.world, &mut context);
    }

    fn draw_gui(&mut self, context: GameContext) {
        let health_bar_rectangle = Rect {
            left: -50.0,
            right: 50.0,
            top: -60.0,
            bottom: -40.0,
        };

        {
            for (_, (transform, health)) in self
                .world
                .query::<(&Transform<f32, f32, f32>, &Health)>()
                .iter()
            {
                let layout = world_to_screen_content_layout(
                    transform.position.xy(),
                    health_bar_rectangle,
                    &context,
                );

                health_bar(layout, health.value as usize);
            }
        }

        content_box(
            ContentBoxItemLayout {
                anchors: Rect {
                    left: 0.0,
                    right: 1.0,
                    top: 1.0,
                    bottom: 1.0,
                },
                margin: Rect {
                    left: 50.0,
                    right: 50.0,
                    top: -100.0,
                    bottom: 50.0,
                },
                align: micro_games_kit::third_party::raui_immediate_widgets::core::Vec2 {
                    x: 0.5,
                    y: 1.0,
                },
                ..Default::default()
            },
            || {
                text_box(TextBoxProps {
                    text: format!("> {}", self.player_controller.spell_text),
                    horizontal_align: TextBoxHorizontalAlign::Center,
                    vertical_align: TextBoxVerticalAlign::Middle,
                    font: TextBoxFont {
                        name: "roboto".to_owned(),
                        size: 48.0,
                    },
                    color: Default::default(),
                    ..Default::default()
                });
            },
        );
    }
}

impl NewGameplay {
    pub fn cast_spell(world: &mut World, cast: PlayerCastAction, caster: Entity) {
        println!("=== CAST SPELL: {:#?}", cast.spell);
        let mut transform = Transform::<f32, f32, f32>::default();
        transform.position = cast.position.into();

        match cast.spell.shape {
            SpellTagShape::Point => Self::cast_point_spell(world, &cast, &transform, caster),
            SpellTagShape::Triangle => Self::cast_triangle_spell(world, &cast, &transform, caster),
            SpellTagShape::Wall => Self::cast_wall_spell(world, &cast, &transform, caster),
        }
    }

    fn cast_wall_spell(
        world: &mut World,
        cast: &PlayerCastAction,
        transform: &Transform<f32, f32, f32>,
        caster: Entity,
    ) {
        let perpendicular_direction = Vec2::new(-cast.direction.y, cast.direction.x);
        let count = 5;
        let start = cast.position
            - perpendicular_direction * cast.spell.size.radius() * (count as f32) / 2.0;

        for i in 0..count {
            let mut new_transform = transform.clone();
            new_transform.position =
                (start + perpendicular_direction * cast.spell.size.radius() * i as f32).into();
            Self::cast_point_spell(world, cast, &new_transform, caster);
        }
    }

    fn cast_triangle_spell(
        world: &mut World,
        cast: &PlayerCastAction,
        transform: &Transform<f32, f32, f32>,
        caster: Entity,
    ) {
        let angle = cast.direction.y.atan2(cast.direction.x);
        let left = angle - std::f32::consts::FRAC_PI_3 + std::f32::consts::FRAC_PI_2;
        let right = angle + std::f32::consts::FRAC_PI_3 + std::f32::consts::FRAC_PI_2;
        let end = angle - std::f32::consts::FRAC_PI_2;
        let left_direction = Vec2::<f32>::new(left.cos(), left.sin());
        let right_direction = Vec2::<f32>::new(right.cos(), right.sin());
        let end_direction = Vec2::<f32>::new(end.cos(), end.sin());
        let count = 5;
        let start = cast.position;

        for i in 0..count {
            let mut new_transform = transform.clone();
            new_transform.position =
                (start + left_direction * cast.spell.size.radius() * i as f32).into();
            Self::cast_point_spell(world, cast, &new_transform, caster);

            if i == 0 {
                continue;
            }

            let mut new_transform = transform.clone();
            new_transform.position =
                (start - right_direction * cast.spell.size.radius() * i as f32).into();
            Self::cast_point_spell(world, cast, &new_transform, caster);
        }

        for i in 0..(count + 1) {
            let mut new_transform = transform.clone();
            new_transform.position = ((start
                + left_direction * cast.spell.size.radius() * count as f32)
                + end_direction * cast.spell.size.radius() * i as f32)
                .into();
            Self::cast_point_spell(world, cast, &new_transform, caster);
        }
    }

    fn cast_point_spell(
        world: &mut World,
        cast: &PlayerCastAction,
        transform: &Transform<f32, f32, f32>,
        caster: Entity,
    ) {
        world.spawn((
            Animation { animation: None },
            Effect::from(cast.spell.effect),
            transform.clone(),
            Projectile::new(
                match cast.spell.speed {
                    SpellTagSpeed::Fast => 1000.0,
                    SpellTagSpeed::Medium => 500.0,
                    SpellTagSpeed::Slow => 100.0,
                },
                cast.direction,
            ),
            Collidable {
                space_object: Some(SpaceObject {
                    entity: None,
                    position: Vec2::default(),
                    collider_radius: 10.0,
                }),
            },
            // SpriteData {
            //     texture: match cast.spell.effect {
            //         SpellTagEffect::None => "particle/smoke".into(),
            //         SpellTagEffect::Fire => "particle/fire".into(),
            //         SpellTagEffect::Electric => "particle/electric".into(),
            //         SpellTagEffect::Water => "particle/water".into(),
            //     },
            //     shader: "image".into(),
            //     pivot: 0.5.into(),
            //     tint: Rgba::white(),
            // },
            ParticleGenerator {
                emmission_accumulator: 0.0,
                emmission_time: 0.1,
                texture: cast.spell.effect.texture().into(),
                batch_size: 16,
            },
            Damage { value: 1.0 },
            IgnoreEntity {
                ignore_time: 0.5,
                ignored_entity: caster,
            },
            cast.spell.clone(),
        ));
    }

    // fn update_ambient_music(&mut self) {
    //     // let player_position = self
    //     //     .player
    //     //     .state
    //     //     .read()
    //     //     .unwrap()
    //     //     .sprite
    //     //     .transform
    //     //     .position
    //     //     .xy();
    //     // let factor = self
    //     //     .enemies
    //     //     .values()
    //     //     .map(|enemy| {
    //     //         enemy
    //     //             .state
    //     //             .read()
    //     //             .unwrap()
    //     //             .sprite
    //     //             .transform
    //     //             .position
    //     //             .xy()
    //     //             .distance(player_position)
    //     //     })
    //     //     .min_by(|a, b| a.partial_cmp(b).unwrap())
    //     //     .unwrap_or(INFINITY)
    //     //     .min(300.0) as f64
    //     //     / 300.0;
    //     // let _ = self
    //     //     .music_forest
    //     //     .set_volume(factor * 2.0, Default::default());
    //     // let _ = self
    //     //     .music_battle
    //     //     .set_volume((1.0 - factor) * 2.0, Default::default());
    // }
}
