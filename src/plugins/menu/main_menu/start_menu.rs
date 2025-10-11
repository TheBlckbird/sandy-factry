use bevy::prelude::*;
use bevy_pkv::{GetError, PkvStore};

const VERSION_MISMATCH_POPUP_ID: &str = "version_mismatch";

use crate::{
    game_save_types::{GameSave, LoadedGameSave},
    plugins::menu::{
        GameState, MAIN_TEXT_COLOR, MENU_BACKGROUND, NORMAL_BUTTON, TEXT_COLOR, UiButton,
        get_button_node, get_button_text_font,
        main_menu::{MainMenuScreen, MainMenuState},
        popup::{PopupCloseEvent, ShowPopupEvent},
    },
    saving::SaveKey,
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
                        UiButton,
                        get_button_node(),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuButtonAction::Play,
                        children![(
                            Text::new("Play"),
                            get_button_text_font(),
                            TextColor(TEXT_COLOR),
                        ),]
                    ),
                    (
                        UiButton,
                        get_button_node(),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuButtonAction::HowToPlay,
                        children![(
                            Text::new("How to Play"),
                            get_button_text_font(),
                            TextColor(TEXT_COLOR)
                        )]
                    ),
                    (
                        UiButton,
                        get_button_node(),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuButtonAction::Quit,
                        children![(
                            Text::new("Quit"),
                            get_button_text_font(),
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

    mut show_popup_writer: EventWriter<ShowPopupEvent>,
    mut popup_close_event_reader: EventReader<PopupCloseEvent>,
) {
    if popup_close_event_reader
        .read()
        .any(|event| event.identifier == VERSION_MISMATCH_POPUP_ID)
    {
        start_game(
            &pkv,
            &mut current_game_save,
            &mut game_state,
            &mut main_menu_state,
        );

        // Terminate if the game was loaded because no other options will be readable at that point
        return;
    }

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MainMenuButtonAction::Play => {
                    // retrieve the version of the saved game to potentially warn the player that his save might be corrupted
                    let saved_version: Result<String, GetError> = pkv.get(SaveKey::Version);

                    match saved_version {
                        Ok(saved_version) => {
                            if saved_version != env!("CARGO_PKG_VERSION") {
                                let message = "Mismatched version of save file. If the game crashes, the save file is too old (or new) and needs to be deleted.\nThere is currently no way to update it to newer versions, I'm sorry.";

                                warn!("{message}");
                                show_popup_writer.write(ShowPopupEvent::with_confirm(
                                    message,
                                    VERSION_MISMATCH_POPUP_ID,
                                ));

                                return;
                            }
                        }
                        Err(get_error) => match get_error {
                            GetError::NotFound => {}

                            GetError::ReDbStorageError(_)
                            | GetError::ReDbTransactionError(_)
                            | GetError::ReDbTableError(_) => panic!("A database error occurred"),

                            GetError::MessagePack(_) => {
                                panic!("The version type shouldn't ever change.")
                            }
                        },
                    }

                    start_game(
                        &pkv,
                        &mut current_game_save,
                        &mut game_state,
                        &mut main_menu_state,
                    );
                }
                MainMenuButtonAction::HowToPlay => {
                    main_menu_state.set(MainMenuState::HowToPlay);
                }
            }
        }
    }
}

/// Loads the saved game and starts the main game loop
fn start_game(
    pkv: &PkvStore,
    current_game_save: &mut LoadedGameSave,
    game_state: &mut NextState<GameState>,
    main_menu_state: &mut NextState<MainMenuState>,
) {
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
