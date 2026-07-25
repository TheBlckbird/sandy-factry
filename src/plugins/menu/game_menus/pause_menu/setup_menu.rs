use bevy::prelude::*;

use crate::plugins::{
    menu::{
        GameState, MENU_BACKGROUND, button,
        game_menus::{GameMenuState, pause_menu::GameMenuScreen},
    },
    save::SaveGameMessage,
};

pub fn setup_pause_menu() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        GameMenuScreen

        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            }
            BackgroundColor(MENU_BACKGROUND)

            Children [
                button("Return to Game")
                on(|_event: On<Pointer<Press>>, mut pause_menu_state: ResMut<NextState<GameMenuState>>| {
                    pause_menu_state.set(GameMenuState::Hidden);
                }),

                button("Main Menu")
                on(|_event: On<Pointer<Press>>,
                    mut pause_menu_state: ResMut<NextState<GameMenuState>>,
                    mut game_state: ResMut<NextState<GameState>>,
                    mut save_game_writer: MessageWriter<SaveGameMessage>| {

                    pause_menu_state.set(GameMenuState::Hidden);
                    game_state.set(GameState::MainMenu);
                    save_game_writer.write_default();
                }),

                button("Save")
                on(|_event: On<Pointer<Press>>, mut save_game_writer: MessageWriter<SaveGameMessage>| {
                    save_game_writer.write(SaveGameMessage::with_indicator());
                }),

                button("Quit")
                on(|_event: On<Pointer<Press>>, mut app_exit_events: MessageWriter<AppExit>, mut save_game_writer: MessageWriter<SaveGameMessage>| {
                    app_exit_events.write_default();
                    save_game_writer.write_default();
                }),
            ]
        ]
    }
}
