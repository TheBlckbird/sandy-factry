use bevy::{ecs::spawn::SpawnWith, prelude::*};

use crate::plugins::menu::{
    MAIN_TEXT_COLOR, MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR, get_button_node,
    get_button_text_font,
    popup::{Popup, PopupAction, PopupCloseEvent, PopupIdentifier, ShowPopupEvent},
};

pub fn listen_show_popup_event(
    mut event_reader: EventReader<ShowPopupEvent>,
    mut commands: Commands,
) {
    let Some(show_popup_event) = event_reader.read().next() else {
        return;
    };

    let popup_type = show_popup_event.popup_type;
    let identifier = show_popup_event.identifier.clone();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
        show_popup_event.popup_type,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MENU_BACKGROUND),
            children![
                (
                    Text::new(show_popup_event.message.clone()),
                    TextFont {
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(MAIN_TEXT_COLOR),
                    Node {
                        margin: UiRect::axes(Val::Px(20.0), Val::Px(30.0)),
                        max_width: Val::Px(match show_popup_event.popup_type {
                            Popup::Confirm => 400.0,
                            Popup::OkCancel => 700.0,
                        }),
                        ..default()
                    }
                ),
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        match popup_type {
                            Popup::Confirm => {
                                parent.spawn((
                                    Button,
                                    get_button_node(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    PopupAction::Confirm,
                                    PopupIdentifier(identifier),
                                    children![(
                                        Text::new("Okay"),
                                        get_button_text_font(),
                                        TextColor(TEXT_COLOR),
                                    )],
                                ));
                            }
                            Popup::OkCancel => {
                                parent.spawn((
                                    Button,
                                    get_button_node(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    PopupAction::Ok,
                                    PopupIdentifier(identifier.clone()),
                                    children![(
                                        Text::new("Okay"),
                                        get_button_text_font(),
                                        TextColor(TEXT_COLOR),
                                    )],
                                ));
                                parent.spawn((
                                    Button,
                                    get_button_node(),
                                    BackgroundColor(NORMAL_BUTTON),
                                    PopupAction::Cancel,
                                    PopupIdentifier(identifier),
                                    children![(
                                        Text::new("Cancel"),
                                        get_button_text_font(),
                                        TextColor(TEXT_COLOR),
                                    )],
                                ));
                            }
                        }
                    }))
                ),
            ],
        )],
    ));
}

pub fn listen_close_popup_event(
    mut close_popup: EventReader<PopupCloseEvent>,
    popup: Option<Single<Entity, With<Popup>>>,
    mut commands: Commands,
) {
    if close_popup.is_empty() {
        return;
    }

    close_popup.clear();

    let Some(popup_entity) = popup else {
        return;
    };

    commands.entity(*popup_entity).despawn();
}
