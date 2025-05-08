use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct TilemapSettings {
    pub map_size: TilemapSize,
    pub tile_size: TilemapTileSize,
    pub map_type: TilemapType,
    pub grid_size: TilemapGridSize,
}

impl TilemapSettings {
    pub fn new(
        map_size: TilemapSize,
        tile_size: TilemapTileSize,
        map_type: TilemapType,
        grid_size: TilemapGridSize,
    ) -> Self {
        Self {
            map_size,
            tile_size,
            map_type,
            grid_size,
        }
    }
}

pub struct TilemapSettingsBorrowed<'a> {
    pub map_size: &'a TilemapSize,
    pub tile_size: &'a TilemapTileSize,
    pub map_type: &'a TilemapType,
    pub grid_size: &'a TilemapGridSize,
}

impl<'a> TilemapSettingsBorrowed<'a> {
    pub fn new(
        map_size: &'a TilemapSize,
        tile_size: &'a TilemapTileSize,
        map_type: &'a TilemapType,
        grid_size: &'a TilemapGridSize,
    ) -> Self {
        Self {
            map_size,
            tile_size,
            map_type,
            grid_size,
        }
    }
}

pub fn get_mouse_tilepos(
    camera: &Camera,
    window: &Window,
    camera_transform: &GlobalTransform,
    map_transform: &Transform,
    map_settings: TilemapSettingsBorrowed,
    anchor: &TilemapAnchor,
) -> Option<TilePos> {
    let cursor_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())?;

    let cursor_position = Vec4::from((cursor_position, 0.0, 1.0));
    let cursor_in_map_position = map_transform.compute_matrix().inverse() * cursor_position;

    TilePos::from_world_pos(
        &cursor_in_map_position.xy(),
        map_settings.map_size,
        map_settings.grid_size,
        map_settings.tile_size,
        map_settings.map_type,
        anchor,
    )
}

/// Generic function for generating a tilemap
/// Accepts a marker component and a closure to generate the tile for a specific tile position
pub fn generate_tilemap_layer<F, C>(
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    z: f32,
    map_settings: TilemapSettings,
    marker_component: C,
    mut gen_texture_index: F,
) where
    F: FnMut(TilePos) -> Option<TileTextureIndex>,
    C: Component + Clone + Copy,
{
    let mut tile_storage = TileStorage::empty(map_settings.map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // Loop through the map size to generate each tile
    for x in 0..map_settings.map_size.x {
        for y in 0..map_settings.map_size.y {
            let tile_pos = TilePos { x, y };

            // Try to fenerate the tile
            let maybe_texture_index = gen_texture_index(tile_pos);

            // Check if a tile was generated
            if let Some(texture_index) = maybe_texture_index {
                // Spawn the generated tile and add it to the tile storage and tilemap
                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index,
                            ..default()
                        },
                        marker_component,
                    ))
                    .id();

                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    }

    // Spawn the tilemap itself
    commands.entity(tilemap_entity).insert((
        make_tilemap_bundle(
            TilemapAnchor::Center,
            texture_handle,
            tile_storage,
            z,
            map_settings,
        ),
        marker_component,
    ));
}

/// Creates a new tilemap bundle with sensible defaults
pub fn make_tilemap_bundle(
    anchor: TilemapAnchor,
    texture_handle: Handle<Image>,
    tile_storage: TileStorage,
    z: f32,
    map_settings: TilemapSettings,
) -> TilemapBundle {
    TilemapBundle {
        grid_size: map_settings.grid_size,
        map_type: map_settings.map_type,
        size: map_settings.map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: map_settings.tile_size,
        anchor,
        transform: Transform::from_xyz(0.0, 0.0, z),
        ..Default::default()
    }
}
