use bevy::prelude::*;

use crate::plugins::menu::popup::{PopupAction, PopupCloseEvent, PopupIdentifier};

pub fn check_popup_interaction(
    interaction_query: Query<
        (&Interaction, &PopupAction, &PopupIdentifier),
        (Changed<Interaction>, With<Button>),
    >,
    mut popup_close_event: MessageWriter<PopupCloseEvent>,
) {
    for (interaction, popup_action, identifier) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let event = match popup_action {
            PopupAction::Confirm => PopupCloseEvent::with_confirm((*identifier).clone()),
            PopupAction::Ok => PopupCloseEvent::with_ok((*identifier).clone()),
            PopupAction::Cancel => PopupCloseEvent::with_cancel((*identifier).clone()),
        };

        popup_close_event.write(event);
    }
}
