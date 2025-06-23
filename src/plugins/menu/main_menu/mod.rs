use bevy::prelude::*;

use crate::plugins::menu::{
    GameState, button_system, despawn_screen,
    main_menu::{
        how_to_play::{setup_how_to_play_menu, update_how_to_play_menu},
        start_menu::{setup_main_menu, update_main_menu},
    },
};

pub mod how_to_play;
pub mod start_menu;

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
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnEnter(MainMenuState::Menu), setup_main_menu)
            .add_systems(
                Update,
                update_main_menu.run_if(in_state(MainMenuState::Menu)),
            )
            .add_systems(
                OnExit(MainMenuState::Menu),
                despawn_screen::<MainMenuScreen>,
            )
            .add_systems(OnEnter(MainMenuState::HowToPlay), setup_how_to_play_menu)
            .add_systems(
                Update,
                update_how_to_play_menu.run_if(in_state(MainMenuState::HowToPlay)),
            )
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

#[derive(Component)]
struct MainMenuScreen;

#[derive(Component)]
struct HowToPlayMenu;
