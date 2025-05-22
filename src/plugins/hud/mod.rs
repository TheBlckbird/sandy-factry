use bevy::prelude::*;
use debug::cleanup;

use super::menu::GameState;

mod debug;

#[derive(Component)]
struct CoordinatesText;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), debug::setup)
            .add_systems(
                Update,
                debug::update_coordinates.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}
