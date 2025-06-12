use belt::{cleanup, setup_item_tilemap, update_item_tilemap};
use bevy::prelude::*;

use super::menu::GameState;

mod belt;

#[derive(Component, Clone, Copy)]
pub struct ItemLayer;

// MARK: Plugin
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_item_tilemap)
            .add_systems(
                Update,
                update_item_tilemap.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}
