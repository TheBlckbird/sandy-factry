use bevy::prelude::*;

use crate::plugins::menu::{
    MAIN_TEXT_COLOR, MENU_BACKGROUND, button,
    popup::{Popup, PopupCloseEvent, ShowPopupEvent},
};

pub fn listen_show_popup_event(
    mut event_reader: MessageReader<ShowPopupEvent>,
    mut commands: Commands,
) {
    let Some(show_popup_event) = event_reader.read().next() else {
        return;
    };

    let popup_type = show_popup_event.popup_type;
    let identifier = show_popup_event.identifier.clone();
    let popup_message = show_popup_event.message.clone();

    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8))
        template_value(popup_type)

        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            }
            BackgroundColor(MENU_BACKGROUND)

            Children [
                Text::new(popup_message)
                TextFont {
                    font_size: px(25),
                }
                TextColor(MAIN_TEXT_COLOR)
                Node {
                    margin: UiRect::axes(px(20), px(30)),
                    max_width: px(match show_popup_event.popup_type {
                        Popup::Confirm => 400,
                        Popup::OkCancel => 700,
                    }),
                },

                Node {
                    flex_direction: FlexDirection::Row,
                }
                Children [{popup_buttons(popup_type, identifier)}]
            ]
        ]
    });
}

fn popup_buttons(popup_type: Popup, identifier: String) -> Box<dyn SceneList> {
    match popup_type {
        Popup::Confirm => Box::new(bsn_list![
            button("Okay")
            on(close_observer(identifier))
        ]),

        Popup::OkCancel => Box::new(bsn_list![
            button("Okay")
            on(close_observer(identifier.clone())),

            button("Cancel")
            on(close_observer(identifier)),
        ]),
    }
}

fn close_observer(
    identifier: String,
) -> impl FnMut(On<Pointer<Press>>, MessageWriter<PopupCloseEvent>) + Clone {
    move |_event, mut popup_close_writer| {
        popup_close_writer.write(PopupCloseEvent {
            identifier: identifier.clone(),
        });
    }
}

pub fn listen_close_popup_event(
    mut close_popup: MessageReader<PopupCloseEvent>,
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
