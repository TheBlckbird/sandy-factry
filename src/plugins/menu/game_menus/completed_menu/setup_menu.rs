use bevy::prelude::*;

use crate::plugins::{
    menu::{
        GameState, MENU_BACKGROUND, TEXT_COLOR, button,
        game_menus::{GameMenuState, completed_menu::CompletedMenuScreen},
    },
    save::SaveGameMessage,
};

pub fn setup_completed_menu() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        CompletedMenuScreen
        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            }
            BackgroundColor(MENU_BACKGROUND)
            Children [
                Text::new("Congratulations")
                TextFont {
                    font_size: px(40),
                }
                TextColor(TEXT_COLOR)
                Node {
                    margin: px(30),
                },

                // Display three buttons for each action available from the pause menu:
                // - continue
                // - main menu
                // - quit
                button("Continue Playing")
                on(|_event: On<Pointer<Press>>, mut pause_menu_state: ResMut<NextState<GameMenuState>>, mut save_game_writer: MessageWriter<SaveGameMessage>| {
                    pause_menu_state.set(GameMenuState::Hidden);
                    save_game_writer.write(SaveGameMessage::with_indicator());
                }),

                button("Main Menu")
                on(|_event: On<Pointer<Press>>,
                    mut pause_menu_state: ResMut<NextState<GameMenuState>>,
                    mut game_state: ResMut<NextState<GameState>>,
                    mut save_game_writer: MessageWriter<SaveGameMessage>,
                    | {

                    pause_menu_state.set(GameMenuState::Hidden);
                    game_state.set(GameState::MainMenu);
                    save_game_writer.write_default();
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
