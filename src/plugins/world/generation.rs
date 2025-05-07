use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Simplex};

use crate::helpers::tilemap::make_tilemap_bundle;

use super::{
    Background, BackgroundObject, MAP_SIZE, MAP_TYPE, Middleground, MiddlegroundObject, Seed,
    TILE_SIZE,
};

/// Run the world generation
pub fn generation(mut commands: Commands, asset_server: Res<AssetServer>, seed: Res<Seed>) {
    let background_texture_handle = asset_server.load("background_tiles.png");
    let middleground_texture_handle = asset_server.load("middleground_tiles.png");

    generate_tilemap_layer(
        &mut commands,
        background_texture_handle,
        Background,
        |_| Some(BackgroundObject::DefaultTile.into()),
        0.0,
    );

    generate_tilemap_layer(
        &mut commands,
        middleground_texture_handle,
        Middleground,
        |tile_pos| {
            // The following method isn't exactly the best, but it's enough for this demo.
            // World Gen isn't the focus of this game

            let simplex = Simplex::new(seed.0);

            // Scale the coordinates to control patch frequency
            let scale = 0.1; // Lower values = larger patches
            let noise_value = simplex.get([tile_pos.x as f64 * scale, tile_pos.y as f64 * scale]);

            // Threshold to determine if the tile has a resource
            if noise_value > 0.5 {
                // Use another noise layer to determine the resource type
                let resource_noise = simplex.get([
                    (tile_pos.x as f64 + 100.0) * scale,
                    (tile_pos.y as f64 + 100.0) * scale,
                ]);

                let middleground_object = if resource_noise < -0.3 {
                    MiddlegroundObject::Iron
                } else if resource_noise < 0.3 {
                    MiddlegroundObject::Copper
                } else {
                    MiddlegroundObject::Coal
                };

                Some(middleground_object.into())
            } else {
                None // No resource in this tile
            }
        },
        1.0,
    );
}

/// Generic function for generating a tilemap
/// Accepts a marker component and a closure to generate the tile for a specific tile position
fn generate_tilemap_layer<F, C>(
    commands: &mut Commands,
    tiles_texture: Handle<Image>,
    make_marker_component: C,
    mut gen_texture_index: F,
    z: f32,
) where
    F: FnMut(TilePos) -> Option<TileTextureIndex>,
    C: Component + Clone + Copy,
{
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();

    // Loop through the map size to generate each tile
    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y {
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
                        make_marker_component,
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
            get_tilemap_center_transform(&MAP_SIZE, &TILE_SIZE.into(), &MAP_TYPE, z),
            tiles_texture,
            tile_storage,
        ),
        make_marker_component,
    ));
}
