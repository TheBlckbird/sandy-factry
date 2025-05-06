use bevy::prelude::*;

mod debug;

#[derive(Component)]
struct CoordinatesText;

#[derive(Component)]
struct HudCamera;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug::setup)
            .add_systems(Update, debug::update_coordinates);
    }
}
