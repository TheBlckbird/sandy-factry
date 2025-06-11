use bevy::prelude::*;

use crate::plugins::menu::{
    button_system, despawn_screen,
    game_menus::{
        GameMenuState,
        pause_menu::{setup_menu::setup_pause_menu, update_menu::update_game_menu},
    },
};

mod setup_menu;
mod update_menu;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::Pause), setup_pause_menu)
            .add_systems(
                Update,
                (update_game_menu, button_system).run_if(in_state(GameMenuState::Pause)),
            )
            .add_systems(
                OnExit(GameMenuState::Pause),
                despawn_screen::<GameMenuScreen>,
            );
    }
}

#[derive(Component)]
enum PauseMenuButtonAction {
    BackToGame,
    BackToMainMenu,
    Quit,
}

#[derive(Component)]
struct GameMenuScreen;
