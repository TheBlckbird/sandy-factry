use bevy::prelude::*;

use crate::plugins::{
    menu::GameState,
    save::{
        auto_save::check_auto_save,
        save_indicator::{save_indicator, update_save_indicator},
        save_listener::check_save_event,
    },
};

mod auto_save;
mod save_indicator;
mod save_listener;

// MARK: Plugin
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveGameMessage>()
            .add_systems(
                OnEnter(GameState::Game),
                (setup, auto_save::setup, save_indicator.spawn()),
            )
            .add_systems(
                Update,
                (check_auto_save, update_save_indicator).run_if(in_state(GameState::Game)),
            )
            .add_systems(Last, check_save_event.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), (cleanup, auto_save::cleanup));
    }
}

// MARK: Events

#[derive(Message, Default)]
pub struct SaveGameMessage {
    show_indicator: bool,
}

impl SaveGameMessage {
    pub fn with_indicator() -> Self {
        Self {
            show_indicator: true,
        }
    }
}

// MARK: Resources

#[derive(Resource, Deref, DerefMut)]
struct SaveIndicatorTimer(Timer);

// MARK: Components

#[derive(Component, Clone, Copy, Default)]
struct SaveIndicator;

// MARK: systems

fn setup(mut commands: Commands) {
    commands.insert_resource(SaveIndicatorTimer(Timer::from_seconds(
        1.0,
        TimerMode::Once,
    )));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SaveIndicatorTimer>();
}
