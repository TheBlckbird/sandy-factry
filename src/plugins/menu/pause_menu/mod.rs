use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_pkv::PkvStore;
use save_game::save_game;

use crate::{content::machine_types::Machine, plugins::world::Seed};

use super::{
    GameMenuButtonAction, GameMenuScreen, GameMenuState, GameState, MENU_BACKGROUND, NORMAL_BUTTON,
    TEXT_COLOR,
};

mod save_game;

pub fn show_game_menu(
    current_game_menu_state: Res<State<GameMenuState>>,
    mut game_menu_state: ResMut<NextState<GameMenuState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_game_menu_state.get() {
            GameMenuState::Hidden => {
                game_menu_state.set(GameMenuState::Pause);
            }
            GameMenuState::Pause | GameMenuState::Recipe => {
                game_menu_state.set(GameMenuState::Hidden);
            }
        }
    }
}

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
        GameMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MENU_BACKGROUND),
            children![
                // Display three buttons for each action available from the main menu:
                // - continue
                // - main menu
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    GameMenuButtonAction::BackToGame,
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
                    GameMenuButtonAction::BackToMainMenu,
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
                    GameMenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),]
                ),
            ]
        )],
    ));
}

pub fn update_menu(
    interaction_query: Query<
        (&Interaction, &GameMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_menu_state: ResMut<NextState<GameMenuState>>,
    mut pkv: ResMut<PkvStore>,
    seed: Res<Seed>,
    tile_query: Query<(&TilePos, &TileTextureIndex, &Machine)>,
    camera: Single<&Transform, With<Camera2d>>,
) {
    let mut should_save_game = false;

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameMenuButtonAction::BackToGame => {
                    pause_menu_state.set(GameMenuState::Hidden);
                }
                GameMenuButtonAction::BackToMainMenu => {
                    pause_menu_state.set(GameMenuState::Hidden);
                    game_state.set(GameState::MainMenu);
                    should_save_game = true;
                }
                GameMenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                    should_save_game = true;
                }
            }
        }
    }

    if should_save_game {
        save_game(
            &mut pkv,
            &seed,
            tile_query.iter().collect(),
            camera.into_inner().translation,
        );
    }
}
