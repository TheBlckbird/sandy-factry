use bevy::prelude::*;

use crate::plugins::menu::{
    MAIN_TEXT_COLOR, MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR,
    main_menu::{HowToPlayMenu, MainMenuState},
};

#[derive(Component)]
pub enum HowToPlayMenuAction {
    Back,
}

pub fn setup_how_to_play_menu(mut commands: Commands) {
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
        HowToPlayMenu,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MENU_BACKGROUND),
            children![
                (
                    Text::new("Sandy Fact'ry"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(MAIN_TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                ),
                (
                    Text::new(include_str!("./how-to-play.txt")),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(MAIN_TEXT_COLOR),
                    Node {
                        max_width: Val::Px(600.0),
                        ..default()
                    }
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    HowToPlayMenuAction::Back,
                    children![(
                        Text::new("Back"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
            ]
        )],
    ));
}

pub fn update_how_to_play_menu(
    interaction_query: Query<
        (&Interaction, &HowToPlayMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, how_to_play_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match how_to_play_button_action {
                HowToPlayMenuAction::Back => {
                    main_menu_state.set(MainMenuState::Menu);
                }
            }
        }
    }
}
