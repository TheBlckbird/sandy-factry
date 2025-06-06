use bevy::prelude::*;

use crate::plugins::{
    interaction::selection_marker::{
        despawn_selection_marker, setup_selection_marker, update_selection_marker,
    },
    menu::GameState,
};

mod selection_marker;

pub struct MachineInteractionPlugin;

impl Plugin for MachineInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_selection_marker)
            .add_systems(
                Update,
                update_selection_marker.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_selection_marker);
    }
}

#[derive(Component)]
struct SelectionMarker;
