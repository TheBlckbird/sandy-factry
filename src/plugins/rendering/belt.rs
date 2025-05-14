use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use sandy_factry_helpers::tilemap::{TilemapSettings, generate_tilemap_layer, remove_tile};

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

pub fn update_item_tilemap(
    mut commands: Commands,
    tilemap_q: Single<(Entity, &mut TileStorage), With<ItemLayer>>,
    forground_tiles: Query<(&TilePos, &TileTextureIndex, &Machine), With<Foreground>>,
    item_tiles: Query<(Entity, &TilePos), With<Item>>,
) {
    let (tilemap_entity, mut tile_storage) = tilemap_q.into_inner();

    // Track the items currently in world;
    let mut current_items_state = HashMap::new();

    for (item_tile_entity, item_tile_pos) in item_tiles {
        current_items_state.insert(*item_tile_pos, item_tile_entity);
    }

    // List all tiles that need to have an item rendered
    let mut desired_items_state = HashMap::new();

    forground_tiles
        .iter()
        .filter(|&(_, tile_texture_index, _)| {
            // Check if the tile is should render items AND has an item on it
            ForegroundObject::from(*tile_texture_index).should_render_item()
        })
        .for_each(|(tile_pos, _, machine)| {
            // `input_items` and `output_items` together should only have one

            let mut all_items = machine.input_items.all();
            all_items.extend(&machine.output_items);

            match all_items.front() {
                None => {}
                Some(&&item) if all_items.len() == 1 => {
                    desired_items_state.insert(*tile_pos, item);
                }
                Some(_) => {
                    panic!("There should only be one item on renderable machines at any given time")
                }
            }
        });

    // Check which tiles already have a rendered item and check if it's supposed to persist to the next frame
    for (tile_pos, entity) in current_items_state.iter() {
        if !desired_items_state.contains_key(tile_pos) {
            remove_tile(&mut commands, &mut tile_storage, *entity, tile_pos);

            // Remove the tile from desired items state, because it doesn't need to be rendered anymore
            desired_items_state.remove(tile_pos);
        }
    }

    // All the remaining tiles in desired_items_state need to be rendered
    for (tile_pos, item) in desired_items_state.iter() {
        commands.spawn((
            TileBundle {
                position: *tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: (*item).into(),
                ..Default::default()
            },
            *item,
        ));
    }
}
