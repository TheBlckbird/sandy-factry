use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    content::machine_types::Machine,
    plugins::{
        completion::HasCompletedGame,
        menu::{
            GameState,
            game_menus::{GameMenuState, completed_menu::CompletedMenuButtonAction},
        },
        world::Seed,
    },
    save_game::save_game,
};

pub fn update_completed_menu(
    interaction_query: Query<
        (&Interaction, &CompletedMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_menu_state: ResMut<NextState<GameMenuState>>,
    mut pkv: ResMut<PkvStore>,
    seed: Res<Seed>,
    tile_query: Query<(&TilePos, &TileTextureIndex, &Machine)>,
    camera: Single<&Transform, With<Camera2d>>,
    has_completed_game: Res<HasCompletedGame>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                CompletedMenuButtonAction::ContinuePlaying => {
                    pause_menu_state.set(GameMenuState::Hidden);
                }
                CompletedMenuButtonAction::BackToMainMenu => {
                    pause_menu_state.set(GameMenuState::Hidden);
                    game_state.set(GameState::MainMenu);
                }
                CompletedMenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }

    save_game(
        &mut pkv,
        &seed,
        tile_query.iter().collect(),
        camera.into_inner().translation,
        **has_completed_game,
    );
}
