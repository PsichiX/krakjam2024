use super::game_end::GameEnd;
use crate::game::{
    components::{
        animation::Animation,
        collidable::Collidable,
        damage::{Damage, DamageLayer},
        effect::Effect,
        health::Health,
        ignore_entity::IgnoreEntity,
        immobility::Immobility,
        particle_generator::ParticleGenerator,
        sprite_data::SpriteData,
    },
    systems::{
        animation_controller::AnimationController, collision_detector::CollisionDetector,
        damage_dealer::DamageDealer, death::Death, effects_reactions::EffectsReactions,
        enemy_controller::EnemyController, enemy_jump_animation::EnemyJumpAnimation,
        enemy_spawn::EnemySpawn, immobility_controller::ImmobilityController,
        particle_manager::ParticleManager, player_controller::PlayerCastAction,
        slime_color::SlimeColor, spell_controller::SpellController,
    },
    ui::{health_bar::health_bar, world_to_screen_content_layout},
    utils::{
        audio::Audio,
        magic::spell_tag::{
            SpellTag, SpellTagDamage, SpellTagDirection, SpellTagDuration, SpellTagEffect,
            SpellTagShape, SpellTagSize, SpellTagSpeed, SpellTagTrajectory,
        },
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
        kira::sound::static_sound::StaticSoundHandle,
        raui_core::layout::CoordsMappingScaling,
        raui_immediate_widgets::core::{
            containers::content_box, image_box, text_box, Color, ContentBoxItemLayout,
            ImageBoxFrame, ImageBoxImage, ImageBoxImageScaling, ImageBoxMaterial, ImageBoxProps,
            TextBoxFont, TextBoxHorizontalAlign, TextBoxProps, TextBoxVerticalAlign,
        },
        spitfire_draw::{
            sprite::{Sprite, SpriteTexture},
            utils::{Drawable, TextureRef},
        },
        spitfire_glow::{graphics::CameraScaling, renderer::GlowTextureFiltering},
        spitfire_input::{InputActionRef, InputConsume, InputMapping, VirtualAction},
        vek::{Transform, Vec2},
        windowing::event::VirtualKeyCode,
    },
};

pub struct NewGameplay {
    map: [Sprite; 4],
    exit: InputActionRef,
    music: StaticSoundHandle,
    world: World,
    player_controller: PlayerController,
    enemy_spawn: EnemySpawn,
    particle_manager: ParticleManager,
    word_to_spell_tag_database: WordToSpellTagDatabase,
    alive_time_seconds: f32,
    tip_time_seconds: f32,
    tip_content: String,
}

impl Default for NewGameplay {
    fn default() -> Self {
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
            music: Audio::write()
                .write()
                .unwrap()
                .play("music/ambient")
                .unwrap(),
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
                .with("blaze", SpellTag::Effect(SpellTagEffect::Fire))
                .with("bonfire", SpellTag::Effect(SpellTagEffect::Fire))
                .with("inferno", SpellTag::Effect(SpellTagEffect::Fire))
                .with("combustion", SpellTag::Effect(SpellTagEffect::Fire))
                .with("devouring", SpellTag::Effect(SpellTagEffect::Fire))
                .with("luminos", SpellTag::Effect(SpellTagEffect::Fire))
                .with("flame", SpellTag::Effect(SpellTagEffect::Fire))
                .with("ignite", SpellTag::Effect(SpellTagEffect::Fire))
                .with("inflamari", SpellTag::Effect(SpellTagEffect::Fire))
                .with("hell", SpellTag::Effect(SpellTagEffect::Fire))
                .with("spark", SpellTag::Effect(SpellTagEffect::Fire))
                .with("incandescence", SpellTag::Effect(SpellTagEffect::Fire))
                .with("torch", SpellTag::Effect(SpellTagEffect::Fire))
                .with("ember", SpellTag::Effect(SpellTagEffect::Fire))
                .with("flare", SpellTag::Effect(SpellTagEffect::Fire))
                .with("pyro", SpellTag::Effect(SpellTagEffect::Fire))
                .with("warmth", SpellTag::Effect(SpellTagEffect::Fire))
                .with("glow", SpellTag::Effect(SpellTagEffect::Fire))
                .with("radiate", SpellTag::Effect(SpellTagEffect::Fire))
                .with("roast", SpellTag::Effect(SpellTagEffect::Fire))
                .with("furnace", SpellTag::Effect(SpellTagEffect::Fire))
                .with("furnace", SpellTag::Effect(SpellTagEffect::Fire))
                .with("pyromania", SpellTag::Effect(SpellTagEffect::Fire))
                .with("bomb", SpellTag::Effect(SpellTagEffect::Fire))
                .with("explosion", SpellTag::Effect(SpellTagEffect::Fire))
                .with("whisky", SpellTag::Effect(SpellTagEffect::Fire))
                .with("flammenwerfer", SpellTag::Effect(SpellTagEffect::Fire))
                .with("flamethrower", SpellTag::Effect(SpellTagEffect::Fire))
                // Water
                .with("wet", SpellTag::Effect(SpellTagEffect::Water))
                .with("aqua", SpellTag::Effect(SpellTagEffect::Water))
                .with("h2o", SpellTag::Effect(SpellTagEffect::Water))
                .with("water", SpellTag::Effect(SpellTagEffect::Water))
                .with("fluid", SpellTag::Effect(SpellTagEffect::Water))
                .with("sprinkle", SpellTag::Effect(SpellTagEffect::Water))
                .with("drink", SpellTag::Effect(SpellTagEffect::Water))
                .with("cool", SpellTag::Effect(SpellTagEffect::Water))
                .with("moist", SpellTag::Effect(SpellTagEffect::Water))
                .with("moisture", SpellTag::Effect(SpellTagEffect::Water))
                .with("rain", SpellTag::Effect(SpellTagEffect::Water))
                .with("dew", SpellTag::Effect(SpellTagEffect::Water))
                .with("drop", SpellTag::Effect(SpellTagEffect::Water))
                .with("river", SpellTag::Effect(SpellTagEffect::Water))
                .with("sea", SpellTag::Effect(SpellTagEffect::Water))
                .with("lake", SpellTag::Effect(SpellTagEffect::Water))
                .with("ocean", SpellTag::Effect(SpellTagEffect::Water))
                .with("spring", SpellTag::Effect(SpellTagEffect::Water))
                .with("splash", SpellTag::Effect(SpellTagEffect::Water))
                .with("shower", SpellTag::Effect(SpellTagEffect::Water))
                .with("soak", SpellTag::Effect(SpellTagEffect::Water))
                .with("hydrate", SpellTag::Effect(SpellTagEffect::Water))
                .with("rinse", SpellTag::Effect(SpellTagEffect::Water))
                .with("bathe", SpellTag::Effect(SpellTagEffect::Water))
                .with("pool", SpellTag::Effect(SpellTagEffect::Water))
                .with("swamp", SpellTag::Effect(SpellTagEffect::Water))
                .with("fountain", SpellTag::Effect(SpellTagEffect::Water))
                .with("vapor", SpellTag::Effect(SpellTagEffect::Water))
                .with("ice", SpellTag::Effect(SpellTagEffect::Water))
                .with("freshwater", SpellTag::Effect(SpellTagEffect::Water))
                .with("liquid", SpellTag::Effect(SpellTagEffect::Water))
                .with("liquor", SpellTag::Effect(SpellTagEffect::Water))
                .with("juice", SpellTag::Effect(SpellTagEffect::Water))
                .with("elixir", SpellTag::Effect(SpellTagEffect::Water))
                .with("potion", SpellTag::Effect(SpellTagEffect::Water))
                .with("emulsion", SpellTag::Effect(SpellTagEffect::Water))
                .with("oil", SpellTag::Effect(SpellTagEffect::Water))
                .with("brew", SpellTag::Effect(SpellTagEffect::Water))
                .with("spill", SpellTag::Effect(SpellTagEffect::Water))
                .with("alco", SpellTag::Effect(SpellTagEffect::Water))
                .with("alcohol", SpellTag::Effect(SpellTagEffect::Water))
                .with("vodka", SpellTag::Effect(SpellTagEffect::Water))
                // Electricity
                .with("zap", SpellTag::Effect(SpellTagEffect::Electric))
                .with("power", SpellTag::Effect(SpellTagEffect::Electric))
                .with("tingly", SpellTag::Effect(SpellTagEffect::Electric))
                .with("charged", SpellTag::Effect(SpellTagEffect::Electric))
                .with("electro", SpellTag::Effect(SpellTagEffect::Electric))
                .with("electric", SpellTag::Effect(SpellTagEffect::Electric))
                .with("thunder", SpellTag::Effect(SpellTagEffect::Electric))
                .with("overcharged", SpellTag::Effect(SpellTagEffect::Electric))
                .with("overcharge", SpellTag::Effect(SpellTagEffect::Electric))
                .with("battery", SpellTag::Effect(SpellTagEffect::Electric))
                .with("light", SpellTag::Effect(SpellTagEffect::Electric))
                .with("lightning", SpellTag::Effect(SpellTagEffect::Electric))
                // Large
                .with("big", SpellTag::Size(SpellTagSize::Large))
                .with("large", SpellTag::Size(SpellTagSize::Large))
                .with("enormous", SpellTag::Size(SpellTagSize::Large))
                .with("chungus", SpellTag::Size(SpellTagSize::Large))
                .with("chonker", SpellTag::Size(SpellTagSize::Large))
                .with("chonk", SpellTag::Size(SpellTagSize::Large))
                .with("humongus", SpellTag::Size(SpellTagSize::Large))
                .with("meteor", SpellTag::Size(SpellTagSize::Large))
                .with("up", SpellTag::Size(SpellTagSize::Large))
                .with("bomb", SpellTag::Size(SpellTagSize::Large))
                .with("explosion", SpellTag::Size(SpellTagSize::Large))
                .with("inferno", SpellTag::Size(SpellTagSize::Large))
                .with("hell", SpellTag::Size(SpellTagSize::Large))
                .with("rain", SpellTag::Size(SpellTagSize::Large))
                .with("mega", SpellTag::Size(SpellTagSize::Large))
                .with("giga", SpellTag::Size(SpellTagSize::Large))
                .with("tera", SpellTag::Size(SpellTagSize::Large))
                .with("gigachad", SpellTag::Size(SpellTagSize::Large))
                // Medium
                .with("fine", SpellTag::Size(SpellTagSize::Medium))
                .with("basic", SpellTag::Size(SpellTagSize::Medium))
                .with("boring", SpellTag::Size(SpellTagSize::Medium))
                .with("blaze", SpellTag::Size(SpellTagSize::Medium))
                .with("compact", SpellTag::Size(SpellTagSize::Medium))
                .with("kilo", SpellTag::Size(SpellTagSize::Medium))
                // Small
                .with("tiny", SpellTag::Size(SpellTagSize::Small))
                .with("cute", SpellTag::Size(SpellTagSize::Small))
                .with("smol", SpellTag::Size(SpellTagSize::Small))
                .with("itsy", SpellTag::Size(SpellTagSize::Small))
                .with("little", SpellTag::Size(SpellTagSize::Small))
                .with("small", SpellTag::Size(SpellTagSize::Small))
                .with("joke", SpellTag::Size(SpellTagSize::Small))
                .with("spark", SpellTag::Size(SpellTagSize::Small))
                .with("mini", SpellTag::Size(SpellTagSize::Small))
                .with("miniature", SpellTag::Size(SpellTagSize::Small))
                .with("mili", SpellTag::Size(SpellTagSize::Small))
                .with("micro", SpellTag::Size(SpellTagSize::Small))
                .with("nano", SpellTag::Size(SpellTagSize::Small))
                .with("pico", SpellTag::Size(SpellTagSize::Small))
                .with("baby", SpellTag::Size(SpellTagSize::Small))
                .with("toy", SpellTag::Size(SpellTagSize::Small))
                .with("teeny", SpellTag::Size(SpellTagSize::Small))
                .with("weeny", SpellTag::Size(SpellTagSize::Small))
                .with("dwarf", SpellTag::Size(SpellTagSize::Small))
                // Sinus
                .with("sinus", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("nice", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("shit", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("spark", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("snake", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("rain", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("drunk", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("alcohol", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("alco", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with("turbulent", SpellTag::Trajectory(SpellTagTrajectory::Sinus))
                .with(
                    "flamethrower",
                    SpellTag::Trajectory(SpellTagTrajectory::Sinus),
                )
                .with(
                    "flammenwerfer",
                    SpellTag::Trajectory(SpellTagTrajectory::Sinus),
                )
                // Circle
                .with("circle", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("joke", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("fuck", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("blaze", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("bonfire", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("hell", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                .with("tornado", SpellTag::Trajectory(SpellTagTrajectory::Circle))
                // Slow
                .with("slow", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("turtle", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("boring", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("faster", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("bomb", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("bonfire", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("rain", SpellTag::Speed(SpellTagSpeed::Slow))
                .with("flammenwerfer", SpellTag::Speed(SpellTagSpeed::Slow))
                // Medium
                .with("regular", SpellTag::Speed(SpellTagSpeed::Medium))
                // Fast
                .with("speedy", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("swift", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("rapid", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("nimble", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("brisk", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("turbo", SpellTag::Speed(SpellTagSpeed::Fast))
                .with("express", SpellTag::Speed(SpellTagSpeed::Fast))
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
                .with("bomb", SpellTag::Direction(SpellTagDirection::Down))
                .with("under", SpellTag::Direction(SpellTagDirection::Down))
                .with("freeze", SpellTag::Direction(SpellTagDirection::Down))
                // Shape - Point
                .with("ball", SpellTag::Shape(SpellTagShape::Point))
                .with("sphere", SpellTag::Shape(SpellTagShape::Point))
                .with("basic", SpellTag::Shape(SpellTagShape::Point))
                .with("boring", SpellTag::Shape(SpellTagShape::Point))
                .with("meteor", SpellTag::Shape(SpellTagShape::Point))
                .with("joke", SpellTag::Shape(SpellTagShape::Point))
                // Shape - Wall
                .with("wall", SpellTag::Shape(SpellTagShape::Wall))
                .with("brick", SpellTag::Shape(SpellTagShape::Wall))
                .with("block", SpellTag::Shape(SpellTagShape::Wall))
                .with("rectangle", SpellTag::Shape(SpellTagShape::Wall))
                .with("square", SpellTag::Shape(SpellTagShape::Wall))
                .with("stuck", SpellTag::Shape(SpellTagShape::Wall))
                // Shape - Triangle
                .with("triangle", SpellTag::Shape(SpellTagShape::Triangle))
                .with("triforce", SpellTag::Shape(SpellTagShape::Triangle))
                .with("illuminati", SpellTag::Shape(SpellTagShape::Triangle))
                .with("flammenwerfer", SpellTag::Shape(SpellTagShape::Triangle))
                // Duration - Instant
                .with("instant", SpellTag::Duration(SpellTagDuration::Instant))
                .with("dead", SpellTag::Duration(SpellTagDuration::Instant))
                .with("bluff", SpellTag::Duration(SpellTagDuration::Instant))
                .with("mistake", SpellTag::Duration(SpellTagDuration::Instant))
                .with("error", SpellTag::Duration(SpellTagDuration::Instant))
                // Duration - Quick
                .with("quick", SpellTag::Duration(SpellTagDuration::Quick))
                .with("brief", SpellTag::Duration(SpellTagDuration::Quick))
                .with("moment", SpellTag::Duration(SpellTagDuration::Quick))
                .with("momentary", SpellTag::Duration(SpellTagDuration::Quick))
                .with("short", SpellTag::Duration(SpellTagDuration::Quick))
                .with("express", SpellTag::Duration(SpellTagDuration::Quick))
                .with("spark", SpellTag::Duration(SpellTagDuration::Quick))
                // Duration - Medium
                .with("moment", SpellTag::Duration(SpellTagDuration::Medium))
                .with("joke", SpellTag::Duration(SpellTagDuration::Medium))
                .with("bomb", SpellTag::Duration(SpellTagDuration::Medium))
                // Duration - Long
                .with("fuck", SpellTag::Duration(SpellTagDuration::Long))
                .with("long", SpellTag::Duration(SpellTagDuration::Long))
                .with("length", SpellTag::Duration(SpellTagDuration::Long))
                .with("lengthy", SpellTag::Duration(SpellTagDuration::Long))
                .with("extended", SpellTag::Duration(SpellTagDuration::Long))
                .with("explosion", SpellTag::Duration(SpellTagDuration::Long))
                .with("hell", SpellTag::Duration(SpellTagDuration::Long))
                .with("flammenwerfer", SpellTag::Duration(SpellTagDuration::Long))
                .with("tornado", SpellTag::Duration(SpellTagDuration::Long))
                // Damage - Low
                .with("spark", SpellTag::Damage(SpellTagDamage::Low))
                .with("drop", SpellTag::Damage(SpellTagDamage::Low))
                .with("ball", SpellTag::Damage(SpellTagDamage::Low))
                .with("smol", SpellTag::Damage(SpellTagDamage::Low))
                .with("sprinkle", SpellTag::Damage(SpellTagDamage::Low))
                .with("zap", SpellTag::Damage(SpellTagDamage::Low))
                .with("ping", SpellTag::Damage(SpellTagDamage::Low))
                // Damage - Medium
                .with("meteor", SpellTag::Damage(SpellTagDamage::Medium))
                .with("wall", SpellTag::Damage(SpellTagDamage::Medium))
                .with("chonker", SpellTag::Damage(SpellTagDamage::Medium))
                .with("blaze", SpellTag::Damage(SpellTagDamage::Medium))
                .with("hell", SpellTag::Damage(SpellTagDamage::Medium))
                .with("ignite", SpellTag::Damage(SpellTagDamage::Medium))
                .with("electro", SpellTag::Damage(SpellTagDamage::Medium))
                .with("power", SpellTag::Damage(SpellTagDamage::Medium))
                .with("electric", SpellTag::Damage(SpellTagDamage::Medium))
                .with("ocean", SpellTag::Damage(SpellTagDamage::Medium))
                .with("splash", SpellTag::Damage(SpellTagDamage::Medium))
                .with("potion", SpellTag::Damage(SpellTagDamage::Medium))
                .with("vodka", SpellTag::Damage(SpellTagDamage::Medium))
                .with("super", SpellTag::Damage(SpellTagDamage::Medium))
                .with("very", SpellTag::Damage(SpellTagDamage::Medium))
                .with("ultra", SpellTag::Damage(SpellTagDamage::Medium))
                .with("hiper", SpellTag::Damage(SpellTagDamage::Medium))
                .with("hyper", SpellTag::Damage(SpellTagDamage::Medium))
                .with("bonk", SpellTag::Damage(SpellTagDamage::Medium))
                .with("golden", SpellTag::Damage(SpellTagDamage::Medium))
                .with("dwarf", SpellTag::Damage(SpellTagDamage::Medium))
                .with("fucking", SpellTag::Damage(SpellTagDamage::Medium))
                // Damage - High
                .with("gigachad", SpellTag::Damage(SpellTagDamage::High))
                .with("exquisite", SpellTag::Damage(SpellTagDamage::High))
                .with("turbulent", SpellTag::Damage(SpellTagDamage::High))
                .with("triforce", SpellTag::Damage(SpellTagDamage::High))
                .with("inferno", SpellTag::Damage(SpellTagDamage::High))
                .with("pyromania", SpellTag::Damage(SpellTagDamage::High))
                .with("incandescence", SpellTag::Damage(SpellTagDamage::High))
                .with("inflamari", SpellTag::Damage(SpellTagDamage::High))
                .with("explosion", SpellTag::Damage(SpellTagDamage::High))
                .with("illuminati", SpellTag::Damage(SpellTagDamage::High))
                .with("flammenwerfer", SpellTag::Damage(SpellTagDamage::High))
                .with("thunder", SpellTag::Damage(SpellTagDamage::High))
                .with("overcharged", SpellTag::Damage(SpellTagDamage::High))
                .with("overcharge", SpellTag::Damage(SpellTagDamage::High))
                .with("emulsion", SpellTag::Damage(SpellTagDamage::High))
                .with("moisture", SpellTag::Damage(SpellTagDamage::High))
                .with("liquor", SpellTag::Damage(SpellTagDamage::High)),
            alive_time_seconds: 0.0,
            tip_time_seconds: 0.0,
            tip_content: Default::default(),
        }
    }
}

impl GameState for NewGameplay {
    fn enter(&mut self, context: GameContext) {
        context.graphics.color = [0.0, 0.3, 0.0, 1.0];
        context.graphics.main_camera.screen_alignment = 0.5.into();
        context.graphics.main_camera.scaling = CameraScaling::FitVertical(800.0);
        context.gui.coords_map_scaling = CoordsMappingScaling::FitVertical(1024.0);

        context
            .input
            .push_mapping(InputMapping::default().consume(InputConsume::Hit).action(
                VirtualAction::KeyButton(VirtualKeyCode::Escape),
                self.exit.clone(),
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
            Health {
                value: 100.0,
                limit: 100.0,
                layer: DamageLayer::None,
            },
            Effect {
                electricity: false,
                fire: false,
                water: false,
            },
            SpriteData {
                texture: "player/idle/0".into(),
                ..Default::default()
            },
            Immobility { time_left: 0.0 },
        ));
    }

    fn exit(&mut self, context: GameContext) {
        context.input.pop_mapping();

        let _ = self.music.stop(Default::default());
    }

    fn fixed_update(&mut self, mut context: GameContext, delta_time: f32) {
        if self.exit.get().is_down() {
            *context.state_change = GameStateChange::Pop;
        }

        self.alive_time_seconds += delta_time;
        self.tip_time_seconds -= delta_time;
        if self.tip_time_seconds <= 0.0 {
            self.tip_time_seconds = 2.0;
            self.tip_content = self.word_to_spell_tag_database.random_word().to_uppercase();
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
        ProjectileController::run(&mut self.world, delta_time);
        CollisionDetector::run(&self.world);
        EffectsReactions::run(&mut self.world);
        SpellController::run(&mut self.world);
        DamageDealer::run(&self.world);
        self.particle_manager.process(&mut self.world, delta_time);
        SlimeColor::run(&self.world);
        EnemyJumpAnimation::run(&self.world, delta_time);
        ImmobilityController::run(&self.world, delta_time);

        // always keep death last in the frame to run!
        Death::run(&mut self.world);

        if self.world.query::<&Player>().iter().next().is_none() {
            *context.state_change =
                GameStateChange::Swap(Box::new(GameEnd::new(self.alive_time_seconds)));
        }
    }

    fn draw(&mut self, mut context: GameContext) {
        for sprite in &self.map {
            sprite.draw(context.draw, context.graphics);
        }

        SpriteRenderer::run(&self.world, &mut context);
        self.particle_manager.draw(&self.world, &mut context);
    }

    fn draw_gui(&mut self, context: GameContext) {
        use micro_games_kit::third_party::raui_immediate_widgets::core::{Rect, Vec2};

        let health_bar_rectangle = Rect {
            left: -50.0,
            right: 50.0,
            top: -70.0,
            bottom: -50.0,
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

                health_bar(layout, health.value, health.limit);
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
                    left: 100.0,
                    right: 100.0,
                    top: -150.0,
                    bottom: 10.0,
                },
                align: Vec2 { x: 0.5, y: 1.0 },
                ..Default::default()
            },
            || {
                image_box(ImageBoxProps {
                    material: ImageBoxMaterial::Image(ImageBoxImage {
                        id: "ui/panel".to_owned(),
                        scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
                            source: 0.5.into(),
                            destination: 70.0.into(),
                            frame_only: false,
                            frame_keep_aspect_ratio: false,
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                text_box(TextBoxProps {
                    text: if self.player_controller.spell_text.is_empty() {
                        "Type your spell...".to_owned()
                    } else {
                        self.player_controller.spell_text.to_uppercase()
                    },
                    horizontal_align: TextBoxHorizontalAlign::Center,
                    vertical_align: TextBoxVerticalAlign::Middle,
                    font: TextBoxFont {
                        name: "roboto".to_owned(),
                        size: 48.0,
                    },
                    color: Color {
                        r: 0.9,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    },
                    ..Default::default()
                });
            },
        );

        content_box(
            ContentBoxItemLayout {
                anchors: Rect {
                    left: 0.0,
                    right: 1.0,
                    top: 1.0,
                    bottom: 1.0,
                },
                margin: Rect {
                    left: 300.0,
                    right: 300.0,
                    top: -200.0,
                    bottom: 160.0,
                },
                align: Vec2 { x: 0.5, y: 1.0 },
                ..Default::default()
            },
            || {
                image_box(ImageBoxProps::colored(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                }));

                text_box(TextBoxProps {
                    text: format!("Tip word: {}", self.tip_content),
                    horizontal_align: TextBoxHorizontalAlign::Center,
                    vertical_align: TextBoxVerticalAlign::Middle,
                    font: TextBoxFont {
                        name: "roboto".to_owned(),
                        size: 32.0,
                    },
                    color: Color {
                        r: 0.9,
                        g: 0.9,
                        b: 0.9,
                        a: 1.0,
                    },
                    ..Default::default()
                });
            },
        );

        content_box(
            ContentBoxItemLayout {
                anchors: Rect {
                    left: 0.5,
                    right: 0.5,
                    top: 0.0,
                    bottom: 0.0,
                },
                margin: Rect {
                    left: -300.0,
                    right: -300.0,
                    top: 10.0,
                    bottom: -150.0,
                },
                align: Vec2 { x: 0.5, y: 0.0 },
                ..Default::default()
            },
            || {
                image_box(ImageBoxProps {
                    material: ImageBoxMaterial::Image(ImageBoxImage {
                        id: "ui/panel".to_owned(),
                        scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
                            source: 0.5.into(),
                            destination: 50.0.into(),
                            frame_only: false,
                            frame_keep_aspect_ratio: false,
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

                text_box(TextBoxProps {
                    text: format!("Alive for {:.2} seconds", self.alive_time_seconds),
                    horizontal_align: TextBoxHorizontalAlign::Center,
                    vertical_align: TextBoxVerticalAlign::Middle,
                    font: TextBoxFont {
                        name: "roboto".to_owned(),
                        size: 30.0,
                    },
                    color: Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.9,
                        a: 1.0,
                    },
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
        let mut sound = Audio::write().write().unwrap().play("sound/spell").unwrap();
        let _ = sound.set_volume(0.5, Default::default());

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
                cast.spell.duration.time(),
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
            Damage {
                value: cast.spell.damage.damage(),
                layer: DamageLayer::None,
            },
            IgnoreEntity {
                ignore_time: 0.5,
                ignored_entity: caster,
            },
            cast.spell.clone(),
        ));
    }
}
