use bevy::prelude::*;

use crate::plugins::{
    crafting,
    interaction::{
        machine_clicked::{
            create_recipe_screen, despawn_recipe_screen, update_recipe_screen,
            update_scroll_position,
        },
        selection_marker::{
            despawn_selection_marker, setup_selection_marker, update_selection_marker,
        },
    },
    menu::GameState,
};

mod machine_clicked;
mod selection_marker;

pub struct MachineInteractionPlugin;

impl Plugin for MachineInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                setup_selection_marker,
                create_recipe_screen.after(crafting::startup),
            ),
        )
        .add_systems(
            Update,
            (
                update_selection_marker,
                update_recipe_screen,
                update_scroll_position,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (despawn_selection_marker, despawn_recipe_screen),
        );
    }
}

#[derive(Component)]
struct SelectionMarker;
