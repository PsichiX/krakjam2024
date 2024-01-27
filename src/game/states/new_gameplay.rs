use super::{
    game_end::{GameEnd, GameEndReason},
    main_menu::MainMenu,
};
use crate::game::{
    components::{
        animation::Animation, collidable::Collidable, enemy::Enemy, spell::Spell,
        sprite_data::SpriteData,
    },
    systems::{
        animation_controller::AnimationController, collision_detector::CollisionDetector,
        player_controller::PlayerCastAction,
    },
    utils::magic::spell_tag::SpellTagTrajectory,
};
use crate::game::{
    components::{player::Player, projectile::Projectile},
    player::PlayerState,
    systems::{
        player_controller::PlayerController, projectile_controller::ProjectileController,
        sprite_renderer::SpriteRenderer,
    },
    utils::{
        events::{Event, Events},
        magic::database::WordToSpellTagDatabase,
        space::{Space, SpaceObject, SpaceObjectId},
    },
};
use hecs::World;
use micro_games_kit::{
    animation::{FrameAnimation, NamedAnimation},
    character::Character,
    context::GameContext,
    game::{GameObject, GameState, GameStateChange},
    third_party::{
        raui_core::layout::CoordsMappingScaling,
        raui_immediate_widgets::core::{
            containers::nav_content_box, text_box, ContentBoxItemLayout, Rect, TextBoxFont,
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
    map: Sprite,
    player: Character<PlayerState>,
    // enemies: HashMap<ID<EnemyState>, Character<EnemyState>>,
    // items: HashMap<ID<Item>, Item>,
    // torch: Torch,
    // darkness: Option<Canvas>,
    exit: InputActionRef,
    exit_handle: Option<ID<InputMapping>>,
    // map_radius: f32,
    // music_forest: StaticSoundHandle,
    // music_battle: StaticSoundHandle,
    world: World,
    player_controller: PlayerController,
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
            map: Sprite::single(SpriteTexture {
                sampler: "u_image".into(),
                texture: TextureRef::name("map/level-0"),
                filtering: GlowTextureFiltering::Linear,
            })
            .pivot(0.5.into()),
            player: PlayerState::new_character([0.0, 0.0, 0.0]),
            // enemies: Default::default(),
            // items: Default::default(),
            // torch: Torch::new([0.0, 0.0]),
            // darkness: None,
            exit: Default::default(),
            exit_handle: None,
            // map_radius: 800.0,
            // music_forest,
            // music_battle,
            world: World::new(),
            player_controller: PlayerController::default(),
            word_to_spell_tag_database: WordToSpellTagDatabase::default(),
        }
    }
}

impl GameState for NewGameplay {
    fn enter(&mut self, mut context: GameContext) {
        context.graphics.color = [0.0, 0.3, 0.0, 1.0];
        context.graphics.main_camera.screen_alignment = 0.5.into();
        context.graphics.main_camera.scaling = CameraScaling::FitVertical(512.0);
        context.gui.coords_map_scaling = CoordsMappingScaling::FitVertical(1024.0);

        self.exit_handle = Some(context.input.push_mapping(
            InputMapping::default().consume(InputConsume::Hit).action(
                VirtualAction::KeyButton(VirtualKeyCode::Escape),
                self.exit.clone(),
            ),
        ));

        self.player.activate(&mut context);

        self.player_controller.init(context.input);

        self.world.spawn((
            Player {},
            Animation { animation: None },
            Transform::<f32, f32, f32>::default(),
            Collidable {
                space_object: Some(SpaceObject {
                    entity: None,
                    position: Vec2::default(),
                    collider_radius: 20.0,
                }),
            },
            SpriteData {
                texture: "player/idle/1".into(),
                shader: "character".into(),
                pivot: 0.5.into(),
                tint: Rgba::default(),
            },
        ));

        self.world.spawn((
            Enemy {},
            Animation {
                animation: Some(NamedAnimation {
                    animation: FrameAnimation::new(1..6).fps(10.0).looping().playing(),
                    id: "enemy/idle".to_owned(),
                }),
            },
            Transform::<f32, f32, f32>::default(),
            Collidable {
                space_object: Some(SpaceObject {
                    entity: None,
                    position: Vec2::default(),
                    collider_radius: 10.0,
                }),
            },
            SpriteData {
                texture: "enemy/idle/1".into(),
                shader: "character".into(),
                pivot: [0.25, 0.5].into(),
                tint: Rgba::default(),
            },
        ));

        // for _ in 0..6 {
        //     let position = [
        //         thread_rng().gen_range((-self.map_radius)..=self.map_radius),
        //         thread_rng().gen_range((-self.map_radius)..=self.map_radius),
        //         0.0,
        //     ];
        //     self.enemies.insert(
        //         ID::new(),
        //         EnemyState::new_character(position).activated(&mut context),
        //     );
        // }

        // for _ in 0..20 {
        //     let position = [
        //         thread_rng().gen_range((-self.map_radius)..=self.map_radius),
        //         thread_rng().gen_range((-self.map_radius)..=self.map_radius),
        //     ];
        //     self.items
        //         .insert(ID::new(), Item::new(ItemKind::random(), position));
        // }

        // self.darkness = Some(
        //     Canvas::from_screen(vec![GlowTextureFormat::Monochromatic], context.graphics)
        //         .unwrap()
        //         .color([0.0, 0.0, 0.0, 0.0]),
        // );
    }

    fn exit(&mut self, mut context: GameContext) {
        self.player.deactivate(&mut context);

        // for (_, mut enemy) in self.enemies.drain() {
        //     enemy.deactivate(&mut context);
        // }

        if let Some(id) = self.exit_handle {
            context.input.remove_mapping(id);
            self.exit_handle = None;
        }

        // let _ = self.music_forest.stop(Default::default());
        // let _ = self.music_battle.stop(Default::default());
    }

    fn fixed_update(&mut self, mut context: GameContext, delta_time: f32) {
        Events::maintain(delta_time);

        if self.exit.get().is_down() {
            *context.state_change = GameStateChange::Swap(Box::new(MainMenu));
        }

        self.player_controller.run(
            &mut self.world,
            &mut context,
            delta_time,
            &self.word_to_spell_tag_database,
        );
        AnimationController::run(&self.world, &mut context, delta_time);
        ProjectileController::run(&self.world, &mut context, delta_time);
        CollisionDetector::run(&self.world, &mut context, delta_time);

        // self.process_game_objects(&mut context, delta_time);

        // self.resolve_collisions();

        self.execute_events(&mut context);

        // self.update_ambient_music();
    }

    fn draw(&mut self, mut context: GameContext) {
        self.map.draw(context.draw, context.graphics);

        SpriteRenderer::run(&self.world, &mut context);

        // self.torch.draw(&mut context);

        // for item in self.items.values_mut() {
        //     item.draw(&mut context);
        // }

        // for enemy in self.enemies.values_mut() {
        //     enemy.draw(&mut context);
        // }

        // self.player.draw(&mut context);

        // if let Some(canvas) = &mut self.darkness {
        //     let _ = canvas.match_to_screen(context.graphics);

        //     canvas.with(context.draw, context.graphics, true, |draw, graphics| {
        //         draw_sphere_light(
        //             self.player
        //                 .state
        //                 .read()
        //                 .unwrap()
        //                 .sprite
        //                 .transform
        //                 .position
        //                 .xy(),
        //             200.0,
        //             0.0..=1.0,
        //             1.0,
        //             draw,
        //             graphics,
        //         );

        //         draw_sphere_light(
        //             self.torch.sprite.transform.position.xy(),
        //             350.0,
        //             0.0..=1.0,
        //             1.0,
        //             draw,
        //             graphics,
        //         );
        //     });

        //     Sprite::single(
        //         canvas
        //             .sprite_texture(0, "u_image".into(), GlowTextureFiltering::Linear)
        //             .unwrap(),
        //     )
        //     .pivot([0.0, 1.0].into())
        //     .scale([1.0, -1.0].into())
        //     .shader(ShaderRef::name("lighting"))
        //     .blending(GlowBlending::Alpha)
        //     .screen_space(true)
        //     .uniform(
        //         "u_dark_color".into(),
        //         GlowUniformValue::F4([0.0, 0.0, 0.0, 1.0]),
        //     )
        //     .uniform(
        //         "u_light_color".into(),
        //         GlowUniformValue::F4([0.0, 0.0, 0.0, 0.0]),
        //     )
        //     .draw(context.draw, context.graphics);
        // }
    }

    fn draw_gui(&mut self, context: GameContext) {
        // let health_bar_rectangle = Rect {
        //     left: -50.0,
        //     right: 50.0,
        //     top: -60.0,
        //     bottom: -40.0,
        // };

        // {
        //     let state = self.player.state.read().unwrap();
        //     let layout = world_to_screen_content_layout(
        //         state.sprite.transform.position.xy(),
        //         health_bar_rectangle,
        //         &context,
        //     );

        //     health_bar(layout, state.health);
        // }

        // for enemy in self.enemies.values() {
        //     let state = enemy.state.read().unwrap();
        //     let layout = world_to_screen_content_layout(
        //         state.sprite.transform.position.xy(),
        //         health_bar_rectangle,
        //         &context,
        //     );

        //     health_bar(layout, state.health);
        // }

        // text_box((
        //     ContentBoxItemLayout {
        //         margin: 40.0.into(),
        //         ..Default::default()
        //     },
        //     TextBoxProps {
        //         text: format!(
        //             "Weapon: {:?}\nEnemies: {}\nItems: {}",
        //             self.player.state.read().unwrap().weapon,
        //             self.enemies.len(),
        //             self.items.len(),
        //         ),
        //         font: TextBoxFont {
        //             name: "roboto".to_owned(),
        //             size: 28.0,
        //         },
        //         color: Color {
        //             r: 1.0,
        //             g: 1.0,
        //             b: 1.0,
        //             a: 1.0,
        //         },
        //         ..Default::default()
        //     },
        // ));

        nav_content_box(
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
    pub fn cast_spell(world: &mut World, cast: PlayerCastAction) {
        println!("=== CAST SPELL: {:#?}", cast.spell);
        let mut transform = Transform::<f32, f32, f32>::default();
        transform.position = cast.position.into();

        world.spawn((
            Animation { animation: None },
            transform,
            Projectile {
                speed: 100.0,
                direction: cast.direction,
                trajectory: SpellTagTrajectory::Straight,
            },
            Collidable {
                space_object: Some(SpaceObject {
                    entity: None,
                    position: Vec2::default(),
                    collider_radius: 10.0,
                }),
            },
            SpriteData {
                texture: "item/apple".into(),
                shader: "image".into(),
                pivot: 0.5.into(),
                tint: Rgba::default(),
            },
            cast.spell,
        ));
    }

    fn process_game_objects(&mut self, context: &mut GameContext, delta_time: f32) {
        // self.torch.process(context, delta_time);

        // self.player.process(context, delta_time);

        // for enemy in self.enemies.values_mut() {
        //     enemy.process(context, delta_time);
        //     enemy
        //         .state
        //         .write()
        //         .unwrap()
        //         .sense_player(&self.player.state.read().unwrap());
        // }
    }

    fn execute_events(&mut self, context: &mut GameContext) {
        Events::read(|events| {
            self.player.state.write().unwrap().execute_events(events);

            // for (id, enemy) in &mut self.enemies {
            //     enemy.state.write().unwrap().execute_events(*id, events);
            // }

            for event in events {
                match event {
                    Event::KillPlayer => {
                        *context.state_change =
                            GameStateChange::Swap(Box::new(GameEnd::new(GameEndReason::Lost)));
                    }
                    Event::KillEntity { entity } => {
                        // self.enemies.remove(id);
                        // if self.enemies.is_empty() {
                        //     Events::write_delayed(2.0, Event::WinGame);
                        // }
                        let _ = self.world.despawn(*entity);
                    }
                    Event::KillItem { id } => {
                        // self.items.remove(id);
                    }
                    Event::WinGame => {
                        *context.state_change =
                            GameStateChange::Swap(Box::new(GameEnd::new(GameEndReason::Won)));
                    }
                    _ => {}
                }
            }
        });
    }

    fn update_ambient_music(&mut self) {
        // let player_position = self
        //     .player
        //     .state
        //     .read()
        //     .unwrap()
        //     .sprite
        //     .transform
        //     .position
        //     .xy();
        // let factor = self
        //     .enemies
        //     .values()
        //     .map(|enemy| {
        //         enemy
        //             .state
        //             .read()
        //             .unwrap()
        //             .sprite
        //             .transform
        //             .position
        //             .xy()
        //             .distance(player_position)
        //     })
        //     .min_by(|a, b| a.partial_cmp(b).unwrap())
        //     .unwrap_or(INFINITY)
        //     .min(300.0) as f64
        //     / 300.0;
        // let _ = self
        //     .music_forest
        //     .set_volume(factor * 2.0, Default::default());
        // let _ = self
        //     .music_battle
        //     .set_volume((1.0 - factor) * 2.0, Default::default());
    }

    // fn resolve_collisions(&mut self) {
    //     let space = Space::read();
    //     let space = space.read().unwrap();

    //     for object_item in space.iter() {
    //         if let SpaceObjectId::Item(item_id) = object_item.id {
    //             if let Some(item) = self.items.get(&item_id) {
    //                 for object in space.collisions(object_item, true) {
    //                     match object.id {
    //                         SpaceObjectId::Player => {
    //                             self.player.state.write().unwrap().consume_item(item);
    //                             Events::write(Event::KillItem { id: item_id });
    //                             let _ = Audio::write().write().unwrap().play("collect");
    //                         }
    //                         SpaceObjectId::Enemy(enemy_id) => {
    //                             if let Some(enemy) = self.enemies.get_mut(&enemy_id) {
    //                                 enemy.state.write().unwrap().consume_item(item);
    //                                 Events::write(Event::KillItem { id: item_id });
    //                             }
    //                         }
    //                         _ => {}
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
