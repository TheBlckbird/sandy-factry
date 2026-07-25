use bevy::prelude::*;
use bevy_pkv::{GetError, PkvStore};

const VERSION_MISMATCH_POPUP_ID: &str = "version_mismatch";

use crate::{
    game_save_types::{GameSave, LoadedGameSave},
    plugins::menu::{
        GameState, MAIN_TEXT_COLOR, MENU_BACKGROUND, button,
        main_menu::{MainMenuScreen, MainMenuState},
        popup::{PopupCloseEvent, ShowPopupEvent},
    },
    save_keys::SaveKey,
};

pub fn main_menu() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        MainMenuScreen

        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            }
            BackgroundColor(MENU_BACKGROUND)
            Children [
                // Display the game name
                Text::new("Sandy Fact'ry")
                TextFont {
                    font_size: px(67),
                }
                TextColor(MAIN_TEXT_COLOR)
                Node {
                    margin: px(50),
                },

                button("Play")
                on(play_click),

                button("How to Play")
                on(|_event: On<Pointer<Press>>, mut main_menu_state: ResMut<NextState<MainMenuState>>| {
                    main_menu_state.set(MainMenuState::HowToPlay);
                }),

                button("Quit")
                on(|_event: On<Pointer<Press>>, mut app_exit_events: MessageWriter<AppExit>| {
                    app_exit_events.write_default();
                }),
            ],

            Node {
                position_type: PositionType::Absolute,
                right: px(5),
                bottom: px(5),
            }
            Text::new(format!("v{}", env!("CARGO_PKG_VERSION")))
        ]
    }
}

fn play_click(
    _event: On<Pointer<Press>>,
    pkv: Res<PkvStore>,
    mut show_popup_writer: MessageWriter<ShowPopupEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_game_save: ResMut<LoadedGameSave>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    // retrieve the version of the saved game to potentially warn the player that his save might be corrupted
    let saved_version: Result<String, GetError> = pkv.get(SaveKey::Version);

    match saved_version {
        Ok(saved_version) => {
            if saved_version == env!("CARGO_PKG_VERSION") {
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

pub fn update_main_menu(
    mut popup_close_event_reader: MessageReader<PopupCloseEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut current_game_save: ResMut<LoadedGameSave>,
    pkv: Res<PkvStore>,
) {
    // [TODO] check what this is doing
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
            "An Error occured while trying to load the save state\nTry deleting the save file (/Users/username/Library/Application Support/louisweigel.sandy-factry/bevy_pkv.redb on MacOS).\nThis will delete ALL your save data!"
        ),
    };

    game_state.set(GameState::Game);
    main_menu_state.set(MainMenuState::Hidden);
}
