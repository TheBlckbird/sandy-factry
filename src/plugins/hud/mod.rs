use bevy::prelude::*;

use crate::plugins::{
    hud::{
        coordinates::{coordinates_hud, update_coordinates},
        hovered_item::{hovered_item_hud, update_hovered_item_text},
        information::information_hud,
    },
    menu::GameState,
};

mod coordinates;
mod hovered_item;
mod information;

// MARK: Plugin
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                coordinates_hud.spawn(),
                hovered_item_hud.spawn(),
                information_hud.spawn(),
            ),
        )
        .add_systems(
            Update,
            (update_coordinates, update_hovered_item_text).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (
                coordinates::cleanup,
                hovered_item::cleanup,
                information::cleanup,
            ),
        );
    }
}
