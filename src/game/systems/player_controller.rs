use crate::game::{
    components::{
        animation::Animation, effect::Effect, follow_player::FollowPlayer,
        ignore_entity::IgnoreEntity, immobility::Immobility, particle::Particle, player::Player,
        spell::Spell,
    },
    states::new_gameplay::NewGameplay,
    utils::magic::{database::WordToSpellTagDatabase, spell_tag::SpellTagEffect},
};
use hecs::{Entity, World};
use micro_games_kit::{
    animation::{FrameAnimation, NamedAnimation},
    context::GameContext,
    third_party::{
        spitfire_input::{
            InputActionRef, InputAxisRef, InputContext, InputMapping, VirtualAction, VirtualAxis,
        },
        vek::{Transform, Vec2},
        windowing::event::MouseButton,
    },
};

pub struct PlayerInput {
    pub attack_action: InputActionRef,
    pub mouse_x: InputAxisRef,
    pub mouse_y: InputAxisRef,
}

pub struct PlayerController {
    pub input: Option<PlayerInput>,
    pub run_animation: NamedAnimation,
    pub idle_animation: NamedAnimation,
    pub spell_text: String,
    pub walk_area: f32,
}

pub struct PlayerCastAction {
    pub position: Vec2<f32>,
    pub direction: Vec2<f32>,
    pub spell: Spell,
}

impl Default for PlayerController {
    fn default() -> Self {
        PlayerController {
            input: None,
            run_animation: NamedAnimation {
                animation: FrameAnimation::new(1..17).looping().playing(),
                id: "player/walk".to_owned(),
            },
            idle_animation: NamedAnimation {
                animation: FrameAnimation::new(0..1).looping().playing(),
                id: "player".to_owned(),
            },
            spell_text: Default::default(),
            walk_area: 3500.0,
        }
    }
}

impl PlayerController {
    pub fn init(&mut self, context: &mut InputContext) {
        let attack_action = InputActionRef::default();
        let mouse_x = InputAxisRef::default();
        let mouse_y = InputAxisRef::default();

        self.input = Some(PlayerInput {
            attack_action: attack_action.clone(),
            mouse_x: mouse_x.clone(),
            mouse_y: mouse_y.clone(),
        });

        let mapping = InputMapping::default()
            .action(VirtualAction::MouseButton(MouseButton::Left), attack_action)
            .axis(VirtualAxis::MousePositionX, mouse_x)
            .axis(VirtualAxis::MousePositionY, mouse_y);

        context.push_mapping(mapping);
    }

    pub fn run(
        &mut self,
        world: &mut World,
        context: &mut GameContext,
        delta_time: f32,
        word_to_spell_tag_database: &WordToSpellTagDatabase,
    ) {
        let mut cast_spell = None;

        if let Some(mut characters) = context.input.characters().write() {
            for character in characters.take().chars() {
                if character == '\n' || character == '\r' {
                    cast_spell = word_to_spell_tag_database.parse(&self.spell_text);
                    self.spell_text.clear();
                } else if character == ' ' || character.is_alphanumeric() {
                    self.spell_text.push(character);
                }
            }
        }

        let mut cast_action: Option<(Entity, PlayerCastAction)> = None;
        let mut particles = Vec::<Particle>::new();
        let mut player_moved_vector: Option<Vec2<f32>> = None;

        for (entity, (player, transform, animation, immobility, effect)) in world
            .query::<(
                &mut Player,
                &mut Transform<f32, f32, f32>,
                &mut Animation,
                &Immobility,
                &Effect,
            )>()
            .iter()
        {
            if let Some(input) = self.input.as_ref() {
                let mouse_pos = Vec2::new(input.mouse_x.get().0, input.mouse_y.get().0);
                let diff = (mouse_pos - context.graphics.main_camera.screen_size / 2.0) / 200.0;
                let length = diff.magnitude().min(1.0);
                let movement = diff.try_normalized().unwrap_or_default() * length;

                transform.scale.x = if movement.x > 0.0 { -1.0 } else { 1.0 };

                if movement.magnitude() > 0.5 {
                    player_moved_vector = Some(movement * delta_time * 200.0);

                    if immobility.time_left > 0.0 {
                        player_moved_vector = Some(player_moved_vector.unwrap() * 0.5);
                    }

                    transform.position += player_moved_vector.unwrap();

                    if let Some(named_animation) = animation.animation.as_ref() {
                        if named_animation.id != self.run_animation.id {
                            animation.animation = Some(self.run_animation.clone());
                        }
                    } else {
                        animation.animation = Some(self.run_animation.clone());
                    }
                } else {
                    if let Some(named_animation) = animation.animation.as_ref() {
                        if named_animation.id != self.idle_animation.id {
                            animation.animation = Some(self.idle_animation.clone());
                        }
                    } else {
                        animation.animation = Some(self.idle_animation.clone());
                    }
                }

                if input.attack_action.get().is_pressed() {
                    let basic_spell = Spell::basic();

                    cast_action = Some((
                        entity,
                        PlayerCastAction {
                            position: (transform.position
                                + movement.normalized()
                                    * basic_spell.direction.multiplier()
                                    * 15.0)
                                .into(),
                            direction: movement.normalized(),
                            spell: basic_spell,
                        },
                    ));
                }

                if let Some(spell) = cast_spell.as_ref() {
                    cast_action = Some((
                        entity,
                        PlayerCastAction {
                            position: (transform.position
                                + movement.normalized() * spell.direction.multiplier() * 15.0)
                                .into(),
                            direction: movement.normalized(),
                            spell: spell.clone(),
                        },
                    ));
                }

                transform.position.x = transform
                    .position
                    .x
                    .min(self.walk_area)
                    .max(-self.walk_area);
                transform.position.y = transform
                    .position
                    .y
                    .min(self.walk_area)
                    .max(-self.walk_area);
            }

            player.current_effect_particle_accumulator += delta_time;

            while player.current_effect_particle_accumulator > player.current_effect_particle_time
                && effect.to_effect_tag() != SpellTagEffect::None
            {
                player.current_effect_particle_accumulator = 0.0;

                particles.push(Particle::new(
                    effect.to_effect_tag().texture().into(),
                    transform.position.xy() + Vec2::<f32>::new(0.0, -50.0),
                    Vec2::<f32>::zero(),
                    20.0f32.to_radians(),
                    10.0..=20.0,
                    1.0..=2.0,
                    0.4..=1.0,
                ));
            }

            context.graphics.main_camera.transform.position = transform.position;
        }

        for (_, (particle, _)) in world.query::<(&mut Particle, &FollowPlayer)>().iter() {
            if let Some(v) = player_moved_vector {
                particle.position += v;
            }
        }

        for particle in particles {
            world.spawn((particle, FollowPlayer));
        }

        if let Some(cast) = cast_action {
            NewGameplay::cast_spell(world, cast.1, cast.0);
        }

        for (_, (ignore_player,)) in world.query::<(&mut IgnoreEntity,)>().iter() {
            ignore_player.ignore_time = (ignore_player.ignore_time - delta_time).max(0.0);
        }
    }
}
