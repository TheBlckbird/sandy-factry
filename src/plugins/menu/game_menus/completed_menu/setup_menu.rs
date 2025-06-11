use bevy::prelude::*;

use crate::plugins::menu::{
    MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR,
    game_menus::completed_menu::{CompletedMenuButtonAction, CompletedMenuScreen},
};

pub fn setup_completed_menu(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(350.0),
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
        CompletedMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MENU_BACKGROUND),
            children![
                (
                    Text::new("Congratulations"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    },
                ),
                // Display three buttons for each action available from the pause menu:
                // - continue
                // - main menu
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    CompletedMenuButtonAction::ContinuePlaying,
                    children![(
                        Text::new("Continue Playing"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    CompletedMenuButtonAction::BackToMainMenu,
                    children![(
                        Text::new("Main Menu"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    CompletedMenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),]
                ),
            ]
        )],
    ));
}
