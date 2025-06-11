use bevy::prelude::*;

use crate::{
    content::items::Item,
    plugins::{completion::HasCompletedGame, menu::game_menus::GameMenuState},
};

pub fn check_completion(
    item_tiles: Query<&Item>,
    mut game_menu_state: ResMut<NextState<GameMenuState>>,
    mut has_completed_game: ResMut<HasCompletedGame>,
) {
    for item in item_tiles {
        if item.ends_game() && !**has_completed_game {
            game_menu_state.set(GameMenuState::Completed);
            **has_completed_game = true;
        }
    }
}
