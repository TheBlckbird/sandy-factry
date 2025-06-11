use bevy::prelude::*;

use crate::{
    game_save_types::LoadedGameSave,
    plugins::{
        completion::check_completion::check_completion, menu::GameState,
        simulation::SimulationUpdate,
    },
};

mod check_completion;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HasCompletedGame(bool);

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

fn setup(mut commands: Commands, game_save: Res<LoadedGameSave>) {
    let has_completed = match &**game_save {
        Some(game_save) => game_save.has_completed_game,
        None => false,
    };

    commands.insert_resource(HasCompletedGame(has_completed));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<HasCompletedGame>();
}
