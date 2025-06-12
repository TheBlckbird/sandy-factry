use bevy::prelude::*;

use crate::plugins::{
    hud::{coordinates::update_coordinates, hovered_item::update_hovered_item_text},
    menu::GameState,
};

mod coordinates;
mod hovered_item;

// MARK: Plugin
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (coordinates::setup, hovered_item::setup),
        )
        .add_systems(
            Update,
            (update_coordinates, update_hovered_item_text).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (coordinates::cleanup, hovered_item::cleanup),
        );
    }
}
