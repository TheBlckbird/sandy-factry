use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Simplex};
use sandy_factry_helpers::tilemap::{TilemapSettings, generate_tilemap_layer};

use crate::plugins::{
    RenderLayer,
    world::{
        Background, BackgroundObject, MAP_SIZE, MAP_TYPE, Middleground, MiddlegroundObject, Seed,
        TILE_SIZE,
    },
};

/// Run the world generation
pub fn generation(mut commands: Commands, asset_server: Res<AssetServer>, seed: Res<Seed>) {
    let background_texture_handle = asset_server.load("background_tiles.png");
    let middleground_texture_handle = asset_server.load("middleground_tiles.png");

    // Create the background tilemap full of sand
    generate_tilemap_layer(
        &mut commands,
        background_texture_handle,
        RenderLayer::Background.into(),
        TilemapSettings::new(MAP_SIZE, TILE_SIZE, MAP_TYPE, TILE_SIZE.into()),
        Background,
        |_| Some((BackgroundObject::Sand.into(), Background)),
    );

    // Generate the middlground tilemap for resources
    generate_tilemap_layer(
        &mut commands,
        middleground_texture_handle,
        RenderLayer::Middleground.into(),
        TilemapSettings::new(MAP_SIZE, TILE_SIZE, MAP_TYPE, TILE_SIZE.into()),
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

                Some((middleground_object.into(), Middleground))
            } else {
                None // No resource in this tile
            }
        },
    );
}

pub fn cleanup(
    mut commands: Commands,
    background_tilemap: Single<Entity, (With<Background>, With<TileStorage>)>,
    middleground_tilemap: Single<Entity, (With<Middleground>, With<TileStorage>)>,
) {
    commands.entity(background_tilemap.entity()).despawn();
    commands.entity(middleground_tilemap.entity()).despawn();
}
