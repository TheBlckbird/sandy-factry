use bevy::prelude::*;

use crate::plugins::menu::game_menus::GameMenuState;

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
            _ => {
                game_menu_state.set(GameMenuState::Hidden);
            }
        }
    }
}
