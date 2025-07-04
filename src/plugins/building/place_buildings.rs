use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;
use sandy_factry_helpers::tilemap::{TilemapSettingsBorrowed, get_mouse_tilepos, remove_tile};

use crate::content::machine_types::Machine;

use super::{
    BuildEvent, BuildingInput, BuildingOutput, Foreground, HoverBuilding,
    foreground_objects::CurrentMachine,
};

/// Place buildings and add hover
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
    current_machine: Res<CurrentMachine>,
    mut event_writer: EventWriter<BuildEvent>,
) {
    // Extract all queried components

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

    // Remove hover building
    for (tile_entity, tile_pos, hover, _) in tile_query.iter() {
        if *tile_pos == mouse_tile_pos && hover.is_none() {
            is_other_tile_at_mouse = true;
        } else if hover.is_some() {
            remove_tile(&mut commands, &mut tile_storage, tile_entity, tile_pos);
        }
    }

    if buttons.pressed(MouseButton::Right) {
        // MARK: erasing mode

        for (tile_entity, tile_pos, hover, texture_index) in tile_query.iter() {
            if *tile_pos == mouse_tile_pos && hover.is_none() {
                commands.entity(tile_entity).despawn();
                tile_storage.remove(&mouse_tile_pos);

                event_writer.write(BuildEvent::Deleted(*tile_pos, (*texture_index).into()));
            }
        }

        return;
    }

    // Return if no machine is selected
    let Some(foreground_object) = current_machine.get_current_foreground_object() else {
        return;
    };

    let tile_texture_index = foreground_object
        .try_into()
        .unwrap_or_else(|_| panic!("This machine shouldn't be selectable: {foreground_object:?}"));

    // Don't try to build if there is already a building at the mouse's position
    if is_other_tile_at_mouse {
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        // MARK: building mode
        // Place the current building

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
                    match foreground_object.try_into().ok() {
                        Some(building_type) => building_type,
                        None => return,
                    },
                    foreground_object.get_input_sides().into(),
                    foreground_object.get_output_sides().try_into().ok(),
                ),
                BuildingInput(foreground_object.get_input_sides()),
                BuildingOutput(foreground_object.get_output_sides()),
            ))
            .id();

        event_writer.write(BuildEvent::Placed(
            mouse_tile_pos,
            tile_texture_index.into(),
        ));

        commands.entity(tilemap_entity).add_child(new_tile_entity);
        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    } else {
        // MARK: hover mode
        // Add the hover building

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

        commands.entity(tilemap_entity).add_child(new_tile_entity);
        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    }
}
