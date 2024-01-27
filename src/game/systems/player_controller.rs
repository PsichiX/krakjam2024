use crate::game::{
    components::{animation::Animation, player::Player, spell::Spell},
    states::new_gameplay::NewGameplay,
    utils::magic::database::WordToSpellTagDatabase,
};
use hecs::World;
use micro_games_kit::{
    animation::{FrameAnimation, NamedAnimation},
    context::GameContext,
    third_party::{
        spitfire_input::{
            InputActionRef, InputAxisRef, InputContext, InputMapping, VirtualAction, VirtualAxis,
        },
        vek::{Clamp, Transform, Vec2},
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
                animation: FrameAnimation::new(0..1).looping().playing(),
                id: "player".to_owned(),
            },
            idle_animation: NamedAnimation {
                animation: FrameAnimation::new(0..1).looping().playing(),
                id: "player".to_owned(),
            },
            spell_text: Default::default(),
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

        let mut cast_action: Option<PlayerCastAction> = None;

        for (_, (_, transform, animation)) in world
            .query::<(&Player, &mut Transform<f32, f32, f32>, &mut Animation)>()
            .iter()
        {
            if let Some(input) = self.input.as_ref() {
                let mouse_pos = Vec2::new(input.mouse_x.get().0, input.mouse_y.get().0);
                let diff = (mouse_pos - context.graphics.main_camera.screen_size / 2.0) / 200.0;
                let movement = diff.clamped(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));

                transform.scale.x = if movement.x > 0.0 { -1.0 } else { 1.0 };

                if movement.magnitude() > 0.5 {
                    transform.position += movement * delta_time * 150.0;

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

                    cast_action = Some(PlayerCastAction {
                        position: (transform.position
                            + movement.normalized() * basic_spell.direction.multiplier() * 60.0)
                            .into(),
                        direction: movement.normalized(),
                        spell: basic_spell,
                    });
                }

                if let Some(spell) = cast_spell.as_ref() {
                    cast_action = Some(PlayerCastAction {
                        position: (transform.position
                            + movement.normalized() * spell.direction.multiplier() * 30.0)
                            .into(),
                        direction: movement.normalized(),
                        spell: spell.clone(),
                    });
                }
            }

            context.graphics.main_camera.transform.position = transform.position;
        }

        if let Some(cast) = cast_action {
            NewGameplay::cast_spell(world, cast);
        }
    }
}
