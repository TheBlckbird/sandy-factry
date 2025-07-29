use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    MouseCoordinates,
    plugins::building::{
        Foreground, PickTileEvent,
        foreground_objects::{CurrentMachine, ForegroundObject},
    },
};

/// Listens to the [PickTileEvent] and sets the currently
/// selected building to the tile under the curser
pub fn pick_tile(
    mut event_reader: EventReader<PickTileEvent>,
    mouse_coordinates: Res<MouseCoordinates>,
    foreground_tiles: Query<(&TilePos, &TileTextureIndex), With<Foreground>>,
    mut current_building: ResMut<CurrentMachine>,
) {
    if event_reader.is_empty() {
        return;
    }

    event_reader.clear();

    let mapybe_hovered_foreground_object = foreground_tiles
        .iter()
        .find(|&(&tile_pos, _)| tile_pos == mouse_coordinates.as_tile_pos())
        .map(|(_, tile_texture_index)| ForegroundObject::from(*tile_texture_index));

    if let Some(hovered_foreground_object) = mapybe_hovered_foreground_object {
        let (machine_index, variant_index) = hovered_foreground_object.get_machine_indices();

        // The select_nth methods accept an index starting at 1
        // Why did I do that...
        current_building.select_nth_machine(machine_index + 1);
        current_building.select_nth_variant(variant_index + 1);
    }
}
