use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn get_mouse_tilepos(
    camera: &Camera,
    window: &Window,
    camera_transform: &GlobalTransform,
    map_transform: &Transform,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
) -> Option<TilePos> {
    let cursor_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())?;

    let cursor_position = Vec4::from((cursor_position, 0.0, 1.0));
    let cursor_in_map_position = map_transform.compute_matrix().inverse() * cursor_position;

    TilePos::from_world_pos(&cursor_in_map_position.xy(), map_size, grid_size, map_type)
}

pub fn chunk_pos_to_world_pos(
    position_in_chunk: &TilePos,
    chunk_position: &IVec2,
    chunk_size: UVec2,
) -> TilePos {
    TilePos::new(
        chunk_position.x as u32 * chunk_size.x + position_in_chunk.x,
        chunk_position.y as u32 * chunk_size.y + position_in_chunk.y,
    )
}
