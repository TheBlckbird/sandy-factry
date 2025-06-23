use bevy::prelude::*;

use crate::plugins::{auto_save::save::check_auto_save, menu::GameState};

mod save;

// MARK: Plugin
pub struct AutoSavePlugin;

impl Plugin for AutoSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, check_auto_save.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

// MARK: Resources

#[derive(Resource, Deref, DerefMut)]
struct AutoSaveTimer(Timer);

// MARK: systems

fn setup(mut commands: Commands) {
    commands.insert_resource(AutoSaveTimer(Timer::from_seconds(
        120.0,
        TimerMode::Repeating,
    )));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<AutoSaveTimer>();
}
