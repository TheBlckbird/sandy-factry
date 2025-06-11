use bevy::prelude::*;

use crate::plugins::interaction::SelectedMachine;

pub fn deselect_machine(
    mut commands: Commands,
    selected_machine: Option<Single<Entity, With<SelectedMachine>>>,
) {
    if let Some(selected_machine) = selected_machine {
        commands
            .entity(selected_machine.entity())
            .remove::<SelectedMachine>();
    }
}
