use bevy::prelude::*;

use crate::plugins::{
    interaction::selection_marker::{
        despawn_selection_marker, hide_selection_marker, setup_selection_marker,
        update_selection_marker,
    },
    menu::{GameState, game_menus::GameMenuState},
};

mod selection_marker;

pub struct MachineInteractionPlugin;

impl Plugin for MachineInteractionPlugin {
    fn build(&self, app: &mut App) {
        app //.init_state::<RecipeMenuState>()
            .add_systems(OnEnter(GameState::Game), setup_selection_marker)
            .add_systems(
                Update,
                (update_selection_marker).run_if(can_interact_with_world),
            )
            .add_systems(OnExit(GameMenuState::Hidden), hide_selection_marker)
            .add_systems(OnExit(GameState::Game), despawn_selection_marker);
    }
}

#[derive(Component)]
struct SelectionMarker;

/// Added to the currently selected machine.
///
/// This component exists one or zero times.
#[derive(Component)]
pub struct SelectedMachine;

/// Condition whether the world can currently be interacted with.
///
/// True if the game is running and no menu is open
pub fn can_interact_with_world(
    game_state: Res<State<GameState>>,
    game_menu_state: Res<State<GameMenuState>>,
) -> bool {
    *game_state == GameState::Game && *game_menu_state == GameMenuState::Hidden
}

/// True if the game is running and not in the pause menu
pub fn game_not_paused(
    game_state: Res<State<GameState>>,
    game_menu_state: Res<State<GameMenuState>>,
) -> bool {
    *game_state == GameState::Game && *game_menu_state != GameMenuState::Pause
}
