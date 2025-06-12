use bevy::prelude::*;

use crate::plugins::menu::{
    button_system, despawn_screen,
    game_menus::{
        GameMenuState,
        completed_menu::{setup_menu::setup_completed_menu, update_menu::update_completed_menu},
    },
};

mod setup_menu;
mod update_menu;

// MARK: Plugin
pub struct CompletedMenuPlugin;

impl Plugin for CompletedMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::Completed), setup_completed_menu)
            .add_systems(
                Update,
                (update_completed_menu, button_system).run_if(in_state(GameMenuState::Completed)),
            )
            .add_systems(
                OnExit(GameMenuState::Completed),
                despawn_screen::<CompletedMenuScreen>,
            );
    }
}

// MARK: Components

#[derive(Component)]
struct CompletedMenuScreen;

#[derive(Component)]
enum CompletedMenuButtonAction {
    ContinuePlaying,
    BackToMainMenu,
    Quit,
}
