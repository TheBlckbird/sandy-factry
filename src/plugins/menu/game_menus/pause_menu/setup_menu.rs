use bevy::prelude::*;

use crate::plugins::menu::{
    MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR,
    game_menus::pause_menu::{GameMenuScreen, PauseMenuButtonAction, SaveButtonText},
};

pub fn setup_pause_menu(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

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
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::BackToGame,
                    children![(
                        Text::new("Return to Game"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::BackToMainMenu,
                    children![(
                        Text::new("Main Menu"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::Save,
                    children![(
                        Text::new("Save"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                        SaveButtonText,
                    ),]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    PauseMenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),]
                ),
            ]
        )],
    ));
}
