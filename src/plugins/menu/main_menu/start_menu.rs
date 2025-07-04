use bevy::prelude::*;
use bevy_pkv::{GetError, PkvStore};

use crate::{
    game_save_types::{GameSave, LoadedGameSave},
    plugins::menu::{
        GameState, MAIN_TEXT_COLOR, MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR,
        main_menu::{MainMenuScreen, MainMenuState},
    },
    save_keys::SaveKey,
};

#[derive(Component)]
pub enum MainMenuButtonAction {
    Play,
    Quit,
    HowToPlay,
}

pub fn setup_main_menu(
    mut commands: Commands,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    // Reset `MainMenuState`
    main_menu_state.set(MainMenuState::Menu);

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
        children![
            (
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
                    // Display three buttons for each action available from the main menu:
                    // - play
                    // - quit
                    // - how to play
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
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuButtonAction::HowToPlay,
                        children![(
                            Text::new("How to Play"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR)
                        )]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuButtonAction::Quit,
                        children![(
                            Text::new("Quit"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),]
                    ),
                ]
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                    ..default()
                },
                Text::new(format!("v{}", env!("CARGO_PKG_VERSION")))
            )
        ],
    ));
}

pub fn update_main_menu(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
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
                            "An Error occured while trying to load the save state\nTry to delete the save file (/Users/username/Library/Application Support/louisweigel.sandy-factry/bevy_pkv.redb) on MacOS.\nThis WILL delete all your save data!"
                        ),
                    };

                    game_state.set(GameState::Game);
                    main_menu_state.set(MainMenuState::Hidden);
                }
                MainMenuButtonAction::HowToPlay => {
                    main_menu_state.set(MainMenuState::HowToPlay);
                }
            }
        }
    }
}
