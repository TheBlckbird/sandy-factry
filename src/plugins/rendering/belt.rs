use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use sandy_factry_helpers::tilemap::{TilemapSettings, generate_tilemap_layer, remove_tile};

use crate::{
    content::{items::Item, machine_types::Machine},
    plugins::{
        RenderLayer,
        building::{Foreground, foreground_objects::ForegroundObject},
        rendering::ItemLayer,
        world::{MAP_SIZE, MAP_TYPE, TILE_SIZE},
    },
};

pub fn setup_item_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("item_tiles.png");

    // generate tilemap layer for items
    generate_tilemap_layer(
        &mut commands,
        texture_handle,
        RenderLayer::Items.into(),
        TilemapSettings::new(MAP_SIZE, TILE_SIZE, MAP_TYPE, TILE_SIZE.into()),
        ItemLayer,
        |_| -> Option<(TileTextureIndex, Item)> { None },
    );
}

/// Renders the items on the tilemap when they're on a belt or something similar
pub fn update_item_tilemap(
    mut commands: Commands,
    tilemap_q: Single<(Entity, &mut TileStorage), With<ItemLayer>>,
    foreground_tiles: Query<(&TilePos, &TileTextureIndex, &Machine), With<Foreground>>,
    item_tiles: Query<(Entity, &TilePos, &Item)>,
) {
    let (tilemap_entity, mut tile_storage) = tilemap_q.into_inner();

    // List all tiles that need to have an item rendered
    let mut desired_items_state = HashMap::new();

    foreground_tiles
        .iter()
        .filter(|&(_, tile_texture_index, _)| {
            // Check if the tile is should render items AND has an item on it
            ForegroundObject::from(*tile_texture_index).should_render_item()
        })
        .for_each(|(tile_pos, _, machine)| {
            // `input_items` and `output_items` together should only have one

            let mut all_items = machine.input_items.all();
            all_items.extend(&machine.output_items.all());

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
    for (entity, tile_pos, item) in item_tiles.iter() {
        match desired_items_state.get(tile_pos) {
            Some(desired_item) if item.item_type == desired_item.item_type => {
                desired_items_state.remove(tile_pos);
            }
            _ => remove_tile(&mut commands, &mut tile_storage, entity, tile_pos),
        }
    }

    // All the remaining tiles in desired_items_state need to be rendered
    for (tile_pos, item) in desired_items_state.iter() {
        let tile_entity = commands
            .spawn((
                TileBundle {
                    position: *tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: (**item).into(),
                    ..Default::default()
                },
                *item,
            ))
            .id();

        commands.entity(tilemap_entity).add_child(tile_entity);
        tile_storage.set(tile_pos, tile_entity);
    }
}

pub fn cleanup(
    mut commands: Commands,
    item_tilemap: Single<Entity, (With<TileStorage>, With<ItemLayer>)>,
) {
    commands.entity(item_tilemap.entity()).despawn();
}
