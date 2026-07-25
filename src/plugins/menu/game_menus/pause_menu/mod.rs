use bevy::prelude::*;

use crate::plugins::menu::{
    despawn_screen,
    game_menus::{GameMenuState, pause_menu::setup_menu::setup_pause_menu},
};

mod setup_menu;

// MARK: Plugin
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::Pause), setup_pause_menu.spawn())
            .add_systems(
                OnExit(GameMenuState::Pause),
                despawn_screen::<GameMenuScreen>,
            );
    }
}

// MARK: Components

#[derive(Component, Clone, Copy, Default)]
struct GameMenuScreen;
