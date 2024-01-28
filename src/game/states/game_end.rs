use crate::game::ui::make_theme;
use micro_games_kit::{
    context::GameContext,
    game::{GameState, GameStateChange},
    third_party::{
        raui_immediate::apply_shared_props,
        raui_immediate_widgets::core::{
            containers::nav_vertical_box, image_box, text_box, Color, ImageBoxAspectRatio,
            ImageBoxImage, ImageBoxMaterial, ImageBoxProps, TextBoxFont, TextBoxHorizontalAlign,
            TextBoxProps, TextBoxVerticalAlign, Transform, Vec2,
        },
        spitfire_input::{InputActionRef, InputMapping, VirtualAction},
        windowing::event::{MouseButton, VirtualKeyCode},
    },
};

use super::new_gameplay::NewGameplay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEndReason {
    Lost,
    Won,
}

impl ToString for GameEndReason {
    fn to_string(&self) -> String {
        match self {
            Self::Lost => "YOU LOST".to_owned(),
            Self::Won => "YOU WON".to_owned(),
        }
    }
}

pub struct GameEnd {
    time: f32,
    restart: InputActionRef,
    exit: InputActionRef,
}

impl GameEnd {
    pub fn new(time: f32) -> Self {
        Self {
            time,
            restart: Default::default(),
            exit: Default::default(),
        }
    }
}

impl GameState for GameEnd {
    fn enter(&mut self, context: GameContext) {
        context.graphics.color = [0.2, 0.2, 0.2, 1.0];
        context.gui.coords_map_scaling = Default::default();

        let mapping = InputMapping::default()
            .action(
                VirtualAction::MouseButton(MouseButton::Left),
                self.restart.clone(),
            )
            .action(
                VirtualAction::KeyButton(VirtualKeyCode::Escape),
                self.exit.clone(),
            );

        context.input.push_mapping(mapping);
    }

    fn exit(&mut self, context: GameContext) {
        context.input.pop_mapping();
    }

    fn fixed_update(&mut self, context: GameContext, _delta_time: f32) {
        if self.restart.get().is_pressed() {
            *context.state_change = GameStateChange::Swap(Box::<NewGameplay>::default());
        }
        if self.exit.get().is_pressed() {
            *context.state_change = GameStateChange::Pop;
        }
    }

    fn draw_gui(&mut self, context: GameContext) {
        context.graphics.color = [1.0, 1.0, 1.0, 1.0];

        apply_shared_props(make_theme(), || {
            image_box(ImageBoxProps {
                content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
                    horizontal_alignment: 0.5,
                    vertical_alignment: 0.5,
                    outside: false,
                }),
                material: ImageBoxMaterial::Image(ImageBoxImage {
                    id: "ui/lost".to_owned(),
                    ..Default::default()
                }),
                ..Default::default()
            });

            nav_vertical_box((), || {
                text_box(TextBoxProps {
                    text: format!("Survived: {:.2}", self.time),
                    horizontal_align: TextBoxHorizontalAlign::Center,
                    vertical_align: TextBoxVerticalAlign::Top,
                    font: TextBoxFont {
                        name: "roboto".to_owned(),
                        size: 100.0,
                    },
                    transform: Transform {
                        translation: Vec2 { x: 0.0, y: 100.0 },
                        ..Default::default()
                    },
                    color: Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.9,
                        a: 1.0,
                    },
                    ..Default::default()
                });
            });
        });
    }
}
