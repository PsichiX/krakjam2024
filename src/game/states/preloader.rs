use super::new_gameplay::NewGameplay;
use micro_games_kit::{
    context::GameContext,
    game::{GameState, GameStateChange},
    loader::{load_font, load_shader, load_texture},
    third_party::{
        spitfire_glow::graphics::Shader,
        spitfire_gui::interactions::GuiInteractionsInputs,
        spitfire_input::{
            ArrayInputCombinator, InputActionRef, InputAxisRef, InputConsume, InputMapping,
            VirtualAction, VirtualAxis,
        },
        windowing::event::MouseButton,
    },
};

pub struct Preloader;

impl GameState for Preloader {
    fn enter(&mut self, mut context: GameContext) {
        Self::load_shaders(&mut context);
        Self::load_fonts(&mut context);
        Self::load_textures(&mut context);
        Self::load_sounds_and_music();
        Self::setup_gui_inputs(&mut context);

        *context.state_change = GameStateChange::Swap(Box::new(NewGameplay::default()));
    }
}

impl Preloader {
    fn load_shaders(context: &mut GameContext) {
        load_shader(
            context.draw,
            context.graphics,
            "color",
            Shader::COLORED_VERTEX_2D,
            Shader::PASS_FRAGMENT,
        );
        load_shader(
            context.draw,
            context.graphics,
            "image",
            Shader::TEXTURED_VERTEX_2D,
            Shader::TEXTURED_FRAGMENT,
        );
        load_shader(
            context.draw,
            context.graphics,
            "text",
            Shader::TEXT_VERTEX,
            Shader::TEXT_FRAGMENT,
        );
    }

    fn load_fonts(context: &mut GameContext) {
        load_font(
            context.draw,
            "roboto",
            include_bytes!("../../../assets/fonts/Roboto-Regular.ttf"),
        );
    }

    fn load_textures(context: &mut GameContext) {
        // map
        load_texture(
            context.draw,
            context.graphics,
            "map/tl",
            include_bytes!("../../../assets/maps/map-tl.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "map/tr",
            include_bytes!("../../../assets/maps/map-tr.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "map/bl",
            include_bytes!("../../../assets/maps/map-bl.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "map/br",
            include_bytes!("../../../assets/maps/map-br.png"),
            1,
            1,
        );

        // player character
        load_texture(
            context.draw,
            context.graphics,
            "player/0",
            include_bytes!("../../../assets/images/player/character.png"),
            1,
            1,
        );

        // enemy character
        load_texture(
            context.draw,
            context.graphics,
            "enemy/0",
            include_bytes!("../../../assets/images/enemy/slime.png"),
            1,
            1,
        );

        // particles
        load_texture(
            context.draw,
            context.graphics,
            "particle/fire",
            include_bytes!("../../../assets/images/particles/fire.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/smoke",
            include_bytes!("../../../assets/images/particles/smoke.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/water",
            include_bytes!("../../../assets/images/particles/water.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/drops",
            include_bytes!("../../../assets/images/particles/drops.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/electric",
            include_bytes!("../../../assets/images/particles/electric.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/sparks",
            include_bytes!("../../../assets/images/particles/sparks.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/steam",
            include_bytes!("../../../assets/images/particles/steam.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/paralized",
            include_bytes!("../../../assets/images/particles/paralized.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "particle/explosion",
            include_bytes!("../../../assets/images/particles/explosion.png"),
            1,
            1,
        );

        // ui
        load_texture(
            context.draw,
            context.graphics,
            "ui/panel",
            include_bytes!("../../../assets/images/ui/panel.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "ui/bar",
            include_bytes!("../../../assets/images/ui/bar.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "ui/button/idle",
            include_bytes!("../../../assets/images/ui/button-idle.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "ui/button/select",
            include_bytes!("../../../assets/images/ui/button-select.png"),
            1,
            1,
        );
        load_texture(
            context.draw,
            context.graphics,
            "ui/button/trigger",
            include_bytes!("../../../assets/images/ui/button-trigger.png"),
            1,
            1,
        );
    }

    fn load_sounds_and_music() {
        // let mut audio = Audio::write();
        // let mut audio = audio.write().unwrap();

        // audio.register(
        //     "footstep/grass/1",
        //     include_bytes!("../../../assets/sounds/footstep-grass-1.ogg"),
        // );
        // audio.register(
        //     "footstep/grass/2",
        //     include_bytes!("../../../assets/sounds/footstep-grass-2.ogg"),
        // );
        // audio.register(
        //     "footstep/grass/3",
        //     include_bytes!("../../../assets/sounds/footstep-grass-3.ogg"),
        // );
        // audio.register("sword", include_bytes!("../../../assets/sounds/sword.ogg"));
        // audio.register("axe", include_bytes!("../../../assets/sounds/axe.ogg"));
        // audio.register(
        //     "collect",
        //     include_bytes!("../../../assets/sounds/collect.ogg"),
        // );
        // audio.register(
        //     "button/select",
        //     include_bytes!("../../../assets/sounds/button-select.ogg"),
        // );
        // audio.register(
        //     "button/click",
        //     include_bytes!("../../../assets/sounds/button-click.ogg"),
        // );

        // audio.register("forest", include_bytes!("../../../assets/music/forest.ogg"));
        // audio.register("battle", include_bytes!("../../../assets/music/battle.ogg"));
    }

    fn setup_gui_inputs(context: &mut GameContext) {
        context
            .gui
            .interactions
            .engine
            .deselect_when_no_button_found = true;

        let pointer_x = InputAxisRef::default();
        let pointer_y = InputAxisRef::default();
        let pointer_trigger = InputActionRef::default();

        context.gui.interactions.inputs = GuiInteractionsInputs {
            pointer_position: ArrayInputCombinator::new([pointer_x.clone(), pointer_y.clone()]),
            pointer_trigger: pointer_trigger.clone(),
            ..Default::default()
        };

        context.input.push_mapping(
            InputMapping::default()
                .consume(InputConsume::Hit)
                .axis(VirtualAxis::MousePositionX, pointer_x)
                .axis(VirtualAxis::MousePositionY, pointer_y)
                .action(
                    VirtualAction::MouseButton(MouseButton::Left),
                    pointer_trigger,
                ),
        );
    }
}
