use bevy::prelude::*;
use bevy_pkv::{GetError, PkvStore};

use crate::{
    game_save_types::{GameSave, LoadedGameSave},
    save_keys::SaveKey,
};

use super::{
    GameState, MAIN_TEXT_COLOR, MENU_BACKGROUND, MainMenuButtonAction, MainMenuScreen,
    NORMAL_BUTTON, TEXT_COLOR,
};

pub fn setup_menu(mut commands: Commands) {
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
        MainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MENU_BACKGROUND),
            children![
                // Display the game name
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
                // Display two buttons for each action available from the main menu:
                // - play
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MainMenuButtonAction::Play,
                    children![(
                        Text::new("Play"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MainMenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),]
                ),
            ]
        )],
    ));
}

pub fn update_menu(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_game_save: ResMut<LoadedGameSave>,
    pkv: Res<PkvStore>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::Quit => {
                    // exit the app
                    app_exit_events.write(AppExit::Success);
                }
                MainMenuButtonAction::Play => {
                    // retrieve the saved game
                    let game_save: Result<GameSave, GetError> = pkv.get(SaveKey::GameSave);

                    **current_game_save = match game_save {
                        Ok(game_save) => Some(game_save),
                        Err(GetError::NotFound) => None,
                        _ => panic!(
                            "An Error occured while trying to load the save state\nTry tdo delete the save file (/Users/username/Library/Application Support/louisweigel.sandy-factry/bevy_pkv.redb) on MacOS.\nThis WILL delete all your save data!"
                        ),
                    };

                    game_state.set(GameState::Game);
                }
            }
        }
    }
}
