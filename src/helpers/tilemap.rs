use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::plugins::world::{MAP_SIZE, MAP_TYPE, TILE_SIZE};

pub fn get_mouse_tilepos(
    camera: &Camera,
    window: &Window,
    camera_transform: &GlobalTransform,
    map_transform: &Transform,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    tile_size: &TilemapTileSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Option<TilePos> {
    let cursor_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())?;

    let cursor_position = Vec4::from((cursor_position, 0.0, 1.0));
    let cursor_in_map_position = map_transform.compute_matrix().inverse() * cursor_position;

    TilePos::from_world_pos(
        &cursor_in_map_position.xy(),
        map_size,
        grid_size,
        tile_size,
        map_type,
        anchor,
    )
}

/// Creates a new tilemap bundle with sensible defaults
pub fn make_tilemap_bundle(
    anchor: TilemapAnchor,
    texture_handle: Handle<Image>,
    tile_storage: TileStorage,
    z: f32,
) -> TilemapBundle {
    TilemapBundle {
        grid_size: TILE_SIZE.into(),
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        anchor,
        transform: Transform::from_xyz(0.0, 0.0, z),
        ..Default::default()
    }
}
