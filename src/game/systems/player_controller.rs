use hecs::World;
use micro_games_kit::{animation::{self, FrameAnimation, NamedAnimation}, context::GameContext, third_party::{spitfire_input::{InputAxisRef, InputContext, InputMapping, VirtualAxis}, vek::{Clamp, Transform, Vec2}}};

use crate::game::components::{animation::Animation, player::Player};

pub struct PlayerInput {
    pub mouse_x: InputAxisRef,
    pub mouse_y: InputAxisRef,
}

pub struct PlayerController {
    pub input: Option<PlayerInput>,
    pub run_animation: NamedAnimation,
    pub idle_animation: NamedAnimation
}

impl Default for PlayerController {
    fn default() -> Self {
        PlayerController {
            input: None,
            run_animation: NamedAnimation {
                animation: FrameAnimation::new(1..25)
                    .looping()
                    .playing()
                    .event(6, "footstep")
                    .event(18, "footstep"),
                id: "player/run".to_owned(),
            },
            idle_animation: NamedAnimation {
                animation: FrameAnimation::new(1..2)
                    .looping()
                    .playing(),
                id: "player/idle".to_owned(),
            }
        }
    }
}

impl PlayerController {
    pub fn init(&mut self, context: &mut InputContext) {
        let mouse_x = InputAxisRef::default();
        let mouse_y = InputAxisRef::default();

        self.input = Some(PlayerInput {
            mouse_x: mouse_x.clone(),
            mouse_y: mouse_y.clone()
        });

        let mapping = InputMapping::default()
            .axis(VirtualAxis::MousePositionX, mouse_x)
            .axis(VirtualAxis::MousePositionY, mouse_y);

        context.push_mapping(mapping);
    }

    pub fn run(&self, world: &World, context: &mut GameContext, delta_time: f32) {
        for (_, (_, transform, animation)) in world.query::<(&Player, &mut Transform<f32, f32, f32>, &mut Animation)>().iter() {
            if let Some(input) = self.input.as_ref() {
                let mouse_pos = Vec2::new(input.mouse_x.get().0, input.mouse_y.get().0);
                let diff = (mouse_pos - context.graphics.main_camera.screen_size / 2.0) / 100.0;
                let movement = diff.clamped(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));

                transform.scale.x = if movement.x > 0.0 { -1.0 } else { 1.0 };

                if movement.magnitude() > 0.5 {
                    transform.position += movement * delta_time * 150.0;
                    context.graphics.main_camera.transform.position = transform.position;

                    if let Some(named_animation) = animation.animation.as_ref() {
                        if named_animation.id != self.run_animation.id {
                            animation.animation = Some(self.run_animation.clone());
                        }
                    }
                    else {
                        animation.animation = Some(self.run_animation.clone());
                    }
                }
                else {
                    if let Some(named_animation) = animation.animation.as_ref() {
                        if named_animation.id != self.idle_animation.id {
                            animation.animation = Some(self.idle_animation.clone());
                        }
                    }
                    else {
                        animation.animation = Some(self.idle_animation.clone());
                    }
                }
            }
        }
    }
}