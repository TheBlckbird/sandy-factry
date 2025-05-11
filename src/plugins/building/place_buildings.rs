use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;
use sandy_factry_helpers::tilemap::{TilemapSettingsBorrowed, get_mouse_tilepos};

use crate::machines::{InputItems, Item, Machine};

use super::{
    BuildEvent, BuildingInput, BuildingOutput, CurrentBuilding, Foreground, ForegroundObject,
    HoverBuilding,
};

pub fn place_buildings(
    mut commands: Commands,
    camera_q: Single<(&Camera, &GlobalTransform)>,
    window_q: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    tilemap_q: Single<
        (
            Entity,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &Transform,
            &mut TileStorage,
            &TilemapAnchor,
            &TilemapTileSize,
        ),
        With<Foreground>,
    >,
    tile_query: Query<
        (Entity, &TilePos, Option<&HoverBuilding>, &TileTextureIndex),
        With<Foreground>,
    >,
    current_building: Res<CurrentBuilding>,
    mut event_writer: EventWriter<BuildEvent>,
) {
    let (camera, camera_transform) = camera_q.into_inner();
    let window = window_q.into_inner();

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

    let Some(mouse_tile_pos) = get_mouse_tilepos(
        camera,
        window,
        camera_transform,
        map_transform,
        TilemapSettingsBorrowed::new(map_size, tile_size, map_type, grid_size),
        anchor,
    ) else {
        return;
    };

    let mut is_other_tile_at_mouse = false;

    for (tile_entity, tile_pos, hover, _) in tile_query.iter() {
        if *tile_pos == mouse_tile_pos && hover.is_none() {
            is_other_tile_at_mouse = true;
        } else if hover.is_some() {
            commands.entity(tile_entity).despawn();
            tile_storage.remove(tile_pos);
        }
    }

    if buttons.pressed(MouseButton::Right) {
        // erasing mode

        for (tile_entity, tile_pos, hover, texture_index) in tile_query.iter() {
            if *tile_pos == mouse_tile_pos || hover.is_some() {
                commands.entity(tile_entity).despawn();
                tile_storage.remove(&mouse_tile_pos);

                event_writer.write(BuildEvent::Deleted(*tile_pos, (*texture_index).into()));
            }
        }

        return;
    }

    let foreground_object: ForegroundObject = current_building.as_foreground_object();

    let Ok(tile_texture_index) = foreground_object.try_into() else {
        return;
    };

    if is_other_tile_at_mouse {
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        // building mode

        let new_tile_entity = commands
            .spawn((
                TileBundle {
                    position: mouse_tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_texture_index,
                    ..Default::default()
                },
                Foreground,
                Machine::new(
                    match foreground_object.into_building_type() {
                        Some(building_type) => building_type,
                        None => return,
                    },
                    VecDeque::new(),
                    VecDeque::new(),
                ),
                BuildingInput(foreground_object.get_input_sides()),
                BuildingOutput(foreground_object.get_output_side()),
            ))
            .id();

        event_writer.write(BuildEvent::Placed(
            mouse_tile_pos,
            tile_texture_index.into(),
        ));

        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    } else {
        // hover mode

        let new_tile_entity = commands
            .spawn((
                TileBundle {
                    position: mouse_tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_texture_index,
                    color: TileColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
                    ..Default::default()
                },
                Foreground,
                HoverBuilding,
            ))
            .id();

        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    }
}

// Belt direction

// let neighbors = Neighbors::get_square_neighboring_positions(&mouse_tile_pos, &map_size, false);
// let north_neighbor = neighbors.north;
// let east_neighbor = neighbors.east;
// let south_neighbor = neighbors.south;
// let west_neighbor = neighbors.west;

// let mut connection_options: Vec<(Direction, Direction)> = Vec::new();

// let find_foreground_object = |searched_tile_pos| {
//     let mut foreground_object = None;

//     for (_, tile_pos, _, tile_texture_index) in tile_query.iter() {
//         if *tile_pos == searched_tile_pos {
//             foreground_object = Some(ForegroundObject::from_tile_texture_index(
//                 tile_texture_index,
//             ));
//         }
//     }

//     foreground_object.unwrap()
// };

// let foreground_input_direction = foreground_object.get_input_side();

// let mut north_neighbor_output_direction = None;
// let mut east_neighbor_output_direction = None;
// let mut south_neighbor_output_direction = None;
// let mut west_neighbor_output_direction = None;

// if let Some(north_neighbor) = north_neighbor {
//     let north_foreground_object = find_foreground_object(north_neighbor);
//     north_neighbor_output_direction = north_foreground_object.get_output_side();
// }

// if let Some(east_neighbor) = east_neighbor {
//     let east_foreground_object = find_foreground_object(east_neighbor);
//     east_neighbor_output_direction = east_foreground_object.get_output_side();
// }

// if let Some(south_neighbor) = south_neighbor {
//     let south_foreground_object = find_foreground_object(south_neighbor);
//     south_neighbor_output_direction = south_foreground_object.get_output_side();
// }

// if let Some(west_neighbor) = west_neighbor {
//     let west_foreground_object = find_foreground_object(west_neighbor);
//     west_neighbor_output_direction = west_foreground_object.get_output_side();
// }

// if let Some(north_neighbor_output_direction) = north_neighbor_output_direction {
//     if let Some(foreground_input_direction) = foreground_input_direction {
//         if north_neighbor_output_direction
//         match foreground_input_direction {
//             Direction::North | Direction::South => {}
//             Direction::East => todo!(),
//             Direction::West => todo!(),
//         }
//     }
// }

// todo!();

// Belt direction end
