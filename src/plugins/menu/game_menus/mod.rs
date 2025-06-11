use bevy::prelude::*;

use crate::plugins::menu::{
    GameState,
    game_menus::{
        completed_menu::CompletedMenuPlugin, pause_menu::PauseMenuPlugin,
        recipe_menu::RecipeMenuPlugin, show_game_menu::show_game_menu,
    },
};

mod completed_menu;
mod pause_menu;
mod recipe_menu;
mod show_game_menu;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMenuState {
    #[default]
    Hidden,
    Pause,
    Recipe,
    Completed,
}

pub struct GameMenusPlugin;

impl Plugin for GameMenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PauseMenuPlugin, RecipeMenuPlugin, CompletedMenuPlugin))
            .init_state::<GameMenuState>()
            .add_systems(Update, show_game_menu.run_if(in_state(GameState::Game)));
    }
}
