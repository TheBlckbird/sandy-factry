use bevy::prelude::*;

use crate::plugins::menu::{
    MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR, UiButton,
    game_menus::completed_menu::{CompletedMenuButtonAction, CompletedMenuScreen},
    get_button_node, get_button_text_font,
};

pub fn setup_completed_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
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
                        font_size: FontSize::Px(40.0),
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: px(30).all(),
                        ..default()
                    },
                ),
                // Display three buttons for each action available from the pause menu:
                // - continue
                // - main menu
                // - quit
                (
                    UiButton,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    CompletedMenuButtonAction::ContinuePlaying,
                    children![(
                        Text::new("Continue Playing"),
                        get_button_text_font(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    UiButton,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    CompletedMenuButtonAction::BackToMainMenu,
                    children![(
                        Text::new("Main Menu"),
                        get_button_text_font(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    UiButton,
                    get_button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    CompletedMenuButtonAction::Quit,
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
