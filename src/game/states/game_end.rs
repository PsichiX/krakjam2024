use crate::game::ui::{make_theme, text_button::text_button};
use micro_games_kit::{
    context::GameContext,
    game::{GameState, GameStateChange},
    third_party::{
        raui_immediate::apply_shared_props,
        raui_immediate_widgets::{
            core::{
                containers::{horizontal_box, nav_vertical_box},
                image_box, FlexBoxItemLayout, ImageBoxAspectRatio, ImageBoxImage, ImageBoxMaterial,
                ImageBoxProps, TextBoxVerticalAlign,
            },
            material::{text_paper, TextPaperProps},
        },
        spitfire_input::{InputActionRef, InputMapping, VirtualAction},
        windowing::event::MouseButton,
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
    reason: GameEndReason,
    restart: Option<InputActionRef>,
}

impl GameEnd {
    pub fn new(reason: GameEndReason) -> Self {
        Self {
            reason,
            restart: None,
        }
    }
}

impl GameState for GameEnd {
    fn enter(&mut self, context: GameContext) {
        context.graphics.color = [0.2, 0.2, 0.2, 1.0];
        context.gui.coords_map_scaling = Default::default();

        let action = InputActionRef::default();
        self.restart = Some(action.clone());

        let mapping =
            InputMapping::default().action(VirtualAction::MouseButton(MouseButton::Left), action);

        context.input.push_mapping(mapping);
    }

    fn fixed_update(&mut self, context: GameContext, delta_time: f32) {
        if let Some(action) = self.restart.as_ref() {
            if action.get().is_pressed() {
                *context.state_change = GameStateChange::Swap(Box::<NewGameplay>::default());
            }
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
                    id: match self.reason {
                        GameEndReason::Lost => "ui/lost".to_owned(),
                        GameEndReason::Won => "ui/won".to_owned(),
                    },
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
    }
}
