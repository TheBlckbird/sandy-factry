use bevy::prelude::*;

use crate::plugins::menu::{
    MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR,
    game_menus::pause_menu::{GameMenuScreen, PauseMenuButtonAction, SaveButtonText},
    get_button_node, get_button_text_font,
};

pub fn setup_pause_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        GameMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MENU_BACKGROUND),
            children![
                // Display three buttons for each action available from the pause menu:
                // - continue
                // - main menu
                // - quit
                (
                    Button,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::BackToGame,
                    children![(
                        Text::new("Return to Game"),
                        get_button_text_font(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::BackToMainMenu,
                    children![(
                        Text::new("Main Menu"),
                        get_button_text_font(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::Save,
                    children![(
                        Text::new("Save"),
                        get_button_text_font(),
                        TextColor(TEXT_COLOR),
                        SaveButtonText,
                    ),]
                ),
                (
                    Button,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::Quit,
                    children![(
                        Text::new("Quit"),
                        get_button_text_font(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
            ]
        )],
    ));
}
