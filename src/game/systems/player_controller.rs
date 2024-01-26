use hecs::World;
use micro_games_kit::{context::GameContext, third_party::{spitfire_input::{CardinalInputCombinator, InputActionRef, InputContext, InputMapping, VirtualAction}, vek::{Transform, Vec2}, windowing::event::VirtualKeyCode}};

use crate::game::components::player::Player;

pub struct PlayerInput {
    pub movement: CardinalInputCombinator,
}

pub struct PlayerController {
    pub input: Option<PlayerInput>,
}

impl Default for PlayerController {
    fn default() -> Self {
        PlayerController {
            input: None
        }
    }
}

impl PlayerController {
    pub fn init(&mut self, context: &mut InputContext) {
        let left = InputActionRef::default();
        let right = InputActionRef::default();
        let up = InputActionRef::default();
        let down = InputActionRef::default();

        self.input = Some(PlayerInput {
            movement: CardinalInputCombinator::new(left.clone(), right.clone(), up.clone(), down.clone())
        });

        let mapping = InputMapping::default()
            .action(VirtualAction::KeyButton(VirtualKeyCode::A), left)
            .action(VirtualAction::KeyButton(VirtualKeyCode::D), right)
            .action(VirtualAction::KeyButton(VirtualKeyCode::W), up)
            .action(VirtualAction::KeyButton(VirtualKeyCode::S), down);

        context.push_mapping(mapping);
    }

    pub fn run(&self, world: &World, context: &mut GameContext, delta_time: f32) {
        for (_, (_, transform)) in world.query::<(&Player, &mut Transform<f32, f32, f32>)>().iter() {
            if let Some(input) = self.input.as_ref() {
                let movement = Vec2::<f32>::from(input.movement.get())
                    .try_normalized()
                    .unwrap_or_default();
                
                transform.position += movement * delta_time * 150.0;
            }
        }
    }
}