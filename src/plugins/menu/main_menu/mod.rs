use bevy::prelude::*;

use crate::plugins::menu::{
    GameState, despawn_screen,
    main_menu::{
        how_to_play::how_to_play_menu,
        start_menu::{main_menu, update_main_menu},
    },
};

mod how_to_play;
mod start_menu;

// MARK: Plugin
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .add_systems(
                OnEnter(GameState::MainMenu),
                |mut main_menu_state: ResMut<NextState<MainMenuState>>| {
                    main_menu_state.set(MainMenuState::Menu);
                },
            )
            .add_systems(OnEnter(MainMenuState::Menu), main_menu.spawn())
            .add_systems(
                Update,
                update_main_menu.run_if(in_state(MainMenuState::Menu)),
            )
            .add_systems(
                OnExit(MainMenuState::Menu),
                despawn_screen::<MainMenuScreen>,
            )
            .add_systems(OnEnter(MainMenuState::HowToPlay), how_to_play_menu.spawn())
            .add_systems(
                OnExit(MainMenuState::HowToPlay),
                despawn_screen::<HowToPlayMenu>,
            );
    }
}

// MARK: State

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    #[default]
    Hidden,
    Menu,
    HowToPlay,
}

// MARK: Components

#[derive(Component, Clone, Copy, Default)]
struct MainMenuScreen;

#[derive(Component, Clone, Copy, Default)]
struct HowToPlayMenu;
