use bevy::prelude::*;

use crate::plugins::menu::{
    despawn_screen,
    game_menus::{GameMenuState, completed_menu::setup_menu::setup_completed_menu},
};

mod setup_menu;

// MARK: Plugin
pub struct CompletedMenuPlugin;

impl Plugin for CompletedMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMenuState::Completed),
            setup_completed_menu.spawn(),
        )
        .add_systems(
            OnExit(GameMenuState::Completed),
            despawn_screen::<CompletedMenuScreen>,
        );
    }
}

// MARK: Components

#[derive(Component, Clone, Copy, Default)]
struct CompletedMenuScreen;
