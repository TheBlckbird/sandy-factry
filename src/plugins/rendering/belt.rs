use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use sandy_factry_helpers::tilemap::{
    TilemapQueryData, TilemapSettings, generate_tilemap_layer, remove_tile,
};

use crate::{
    machines::{Item, Machine},
    plugins::{
        building::{Foreground, ForegroundObject},
        world::{MAP_SIZE, MAP_TYPE, TILE_SIZE},
    },
};

use super::ItemLayer;

pub fn setup_item_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("item_tiles.png");

    // generate tilemap layer for items
    generate_tilemap_layer(
        &mut commands,
        texture_handle,
        3.0,
        TilemapSettings::new(MAP_SIZE, TILE_SIZE, MAP_TYPE, TILE_SIZE.into()),
        ItemLayer,
        |_| -> Option<(TileTextureIndex, Item)> { None },
    );
}

#[allow(unused)] // [TODO] Remove this when finished
pub fn update_item_tilemap(
    mut commands: Commands,
    tilemap_q: Single<TilemapQueryData, With<ItemLayer>>,
    forground_tiles: Query<(Entity, &TilePos, &TileTextureIndex, &Machine), With<Foreground>>,
    item_tiles: Query<(Entity, &TilePos), With<Item>>,
) {
    let (
        tilemap_entity,
        map_size,
        grid_size,
        map_type,
        map_transform,
        mut tile_storage,
        anchor,
        tile_size,
    ) = tilemap_q.into_inner();

    // [TODO] Don't despawn everything
    for (item_tile_entity, item_tile_pos) in item_tiles {
        remove_tile(
            &mut commands,
            &mut tile_storage,
            item_tile_entity,
            item_tile_pos,
        );
    }

    forground_tiles
        .iter()
        .filter(|&(_, tile_pos, tile_texture_index, machine)| {
            // Check if the tile is should render items AND has an item on it
            ForegroundObject::from(*tile_texture_index).should_render_item()
                && (machine.input_items.len() == 1 || machine.output_items.len() == 1)
        })
        .for_each(|(tile_entity, tile_pos, tile_texture_index, machine)| {
            // We know that either `input_items` or `output_items` has one item
            // We first check if input_items has one item, if that's not the case take the item from output items
            let item = machine
                .input_items
                .front()
                .or_else(|| machine.output_items.front())
                .unwrap();

            let new_item_tile = commands.spawn((
                TileBundle {
                    position: *tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: (*item).into(),
                    ..Default::default()
                },
                *item,
            ));
        });
}
