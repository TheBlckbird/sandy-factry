use bevy::prelude::*;

use crate::{
    game_save_types::LoadedGameSave,
    plugins::{
        completion::check_completion::check_completion, menu::GameState,
        simulation::SimulationUpdate,
    },
};

mod check_completion;

// MARK: Plugin
pub struct CompletionPlugin;

impl Plugin for CompletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                SimulationUpdate,
                check_completion.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

// MARK: Resources

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HasCompletedGame(bool);

// MARK: Systems

/// Initialize Resources for [CompletionPlugin]
fn setup(mut commands: Commands, game_save: Res<LoadedGameSave>) {
    // Retrieve completed state or set it to false
    let has_completed = match &**game_save {
        Some(game_save) => game_save.has_completed_game,
        None => false,
    };

    // Store completed state
    commands.insert_resource(HasCompletedGame(has_completed));
}

/// Remove the resources for [CompletionPlugin]
fn cleanup(mut commands: Commands) {
    commands.remove_resource::<HasCompletedGame>();
}
