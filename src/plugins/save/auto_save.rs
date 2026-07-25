use bevy::{prelude::*, window::WindowCloseRequested};

use crate::plugins::save::SaveGameMessage;

#[derive(Resource, Deref, DerefMut)]
pub struct AutoSaveTimer(Timer);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(AutoSaveTimer(Timer::from_seconds(
        180.0,
        TimerMode::Repeating,
    )));
}

pub fn check_auto_save(
    mut auto_save_timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
    app_exit_events: MessageReader<AppExit>,
    window_close_events: MessageReader<WindowCloseRequested>,
    mut save_message_writer: MessageWriter<SaveGameMessage>,
) {
    if auto_save_timer.tick(time.delta()).just_finished()
        || !app_exit_events.is_empty()
        || !window_close_events.is_empty()
    {
        save_message_writer.write(SaveGameMessage::with_indicator());
    }
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<AutoSaveTimer>();
}
