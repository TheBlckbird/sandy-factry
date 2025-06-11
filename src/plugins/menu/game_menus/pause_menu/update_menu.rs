use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    content::machine_types::Machine,
    plugins::{
        menu::{
            GameState,
            game_menus::{
                GameMenuState,
                pause_menu::{PauseMenuButtonAction, save_game::save_game},
            },
        },
        world::Seed,
    },
};

pub fn update_game_menu(
    interaction_query: Query<
        (&Interaction, &PauseMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_menu_state: ResMut<NextState<GameMenuState>>,
    mut pkv: ResMut<PkvStore>,
    seed: Res<Seed>,
    tile_query: Query<(&TilePos, &TileTextureIndex, &Machine)>,
    camera: Single<&Transform, With<Camera2d>>,
) {
    let mut should_save_game = false;

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PauseMenuButtonAction::BackToGame => {
                    pause_menu_state.set(GameMenuState::Hidden);
                }
                PauseMenuButtonAction::BackToMainMenu => {
                    pause_menu_state.set(GameMenuState::Hidden);
                    game_state.set(GameState::MainMenu);
                    should_save_game = true;
                }
                PauseMenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                    should_save_game = true;
                }
            }
        }
    }

    if should_save_game {
        save_game(
            &mut pkv,
            &seed,
            tile_query.iter().collect(),
            camera.into_inner().translation,
        );
    }
}
