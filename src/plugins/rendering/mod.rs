use belt::{setup_item_tilemap, update_item_tilemap};
use bevy::prelude::*;

mod belt;

#[derive(Component, Clone, Copy)]
pub struct ItemLayer;

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_item_tilemap)
            .add_systems(Update, update_item_tilemap);
    }
}
