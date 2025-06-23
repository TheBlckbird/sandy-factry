use bevy::prelude::*;

use crate::plugins::{
    auto_save::{
        save::check_auto_save,
        save_indicator::{setup_save_indicator, update_save_indicator},
    },
    menu::GameState,
};

mod save;
mod save_indicator;

// MARK: Plugin
pub struct AutoSavePlugin;

impl Plugin for AutoSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (setup, setup_save_indicator))
            .add_systems(
                Update,
                (check_auto_save, update_save_indicator).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

// MARK: Resources

#[derive(Resource, Deref, DerefMut)]
struct AutoSaveTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct SaveIndicatorTimer(Timer);

// MARK: Components

#[derive(Component)]
struct SaveIndicator;

// MARK: systems

fn setup(mut commands: Commands) {
    commands.insert_resource(AutoSaveTimer(Timer::from_seconds(
        10.0,
        TimerMode::Repeating,
    )));

    commands.insert_resource(SaveIndicatorTimer(Timer::from_seconds(
        2.0,
        TimerMode::Once,
    )));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<AutoSaveTimer>();
    commands.remove_resource::<SaveIndicatorTimer>();
}
