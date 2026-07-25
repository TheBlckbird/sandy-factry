use bevy::prelude::*;

use check_popup_interaction::check_popup_interaction;
use listen_popup_event::listen_show_popup_event;

use crate::plugins::menu::popup::listen_popup_event::listen_close_popup_event;

mod check_popup_interaction;
mod listen_popup_event;

// MARK: Plugin
pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShowPopupEvent>()
            .add_message::<PopupCloseEvent>()
            .add_systems(
                Update,
                (
                    listen_show_popup_event,
                    check_popup_interaction,
                    listen_close_popup_event,
                ),
            );
    }
}

// MARK: Events

/// Event that should be fired to open a popup.
///
/// The [Popup] type defines the type of popup, if it has one or two buttons and the String the message.
#[derive(Message)]
pub struct ShowPopupEvent {
    popup_type: Popup,
    message: String,
    identifier: String,
}

#[allow(unused)]
impl ShowPopupEvent {
    /// Construct a new [ShowPopupEvent] with a type and message.
    pub fn new(
        popup_type: Popup,
        message: impl Into<String>,
        identifier: impl Into<String>,
    ) -> Self {
        Self {
            popup_type,
            message: message.into(),
            identifier: identifier.into(),
        }
    }

    /// Construct a new [ShowPopupEvent] to show a popup with a confirm button and given message.
    pub fn with_confirm(message: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::new(Popup::Confirm, message, identifier)
    }

    /// Construct a new [ShowPopupEvent] to show a popup with an ok and cancel button and a given message.
    pub fn with_ok_cancel(message: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::new(Popup::OkCancel, message, identifier)
    }
}

/// Event that is fired once a popup closes.
#[derive(Message)]
pub struct PopupCloseEvent {
    #[allow(unused)]
    pub action: PopupAction,
    pub identifier: String,
}

impl PopupCloseEvent {
    pub fn with_confirm(identifier: String) -> Self {
        Self {
            action: PopupAction::Confirm,
            identifier,
        }
    }

    pub fn with_ok(identifier: String) -> Self {
        Self {
            action: PopupAction::Ok,
            identifier,
        }
    }

    pub fn with_cancel(identifier: String) -> Self {
        Self {
            action: PopupAction::Cancel,
            identifier,
        }
    }
}

// MARK: Components

#[allow(unused)]
#[derive(Component, Clone, Copy)]
pub enum Popup {
    Confirm,
    OkCancel,
}

#[derive(Component)]
pub enum PopupAction {
    Confirm,
    Ok,
    Cancel,
}

#[derive(Component, Deref, DerefMut)]
pub struct PopupIdentifier(String);
