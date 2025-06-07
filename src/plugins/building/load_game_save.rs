use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use dyn_clone::clone_box;

use crate::{
    game_save_types::LoadedGameSave,
    machines::Machine,
    plugins::building::{BuildEvent, BuildingInput, BuildingOutput, Foreground},
};

pub fn load_game_save(
    mut commands: Commands,
    game_save: Res<LoadedGameSave>,
    foreground_tilemap: Single<(Entity, &mut TileStorage), With<Foreground>>,
    mut event_writer: EventWriter<BuildEvent>,
) {
    let (tilemap_entity, mut tile_storage) = foreground_tilemap.into_inner();

    if let Some(game_save) = &**game_save {
        for (tile_pos, foreground_object, machine_type, input_items, output_items) in
            &game_save.machines
        {
            let new_tile_entity = commands
                .spawn((
                    TileBundle {
                        position: *tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: (*foreground_object)
                            .try_into()
                            .expect("`Nothing` tile found in game save!"),
                        ..Default::default()
                    },
                    Foreground,
                    Machine::new(
                        clone_box(&**machine_type),
                        input_items.clone(),
                        output_items.clone(),
                    ),
                    BuildingInput(foreground_object.get_input_sides()),
                    BuildingOutput(foreground_object.get_output_sides()),
                ))
                .id();

            event_writer.write(BuildEvent::Placed(*tile_pos, *foreground_object));

            commands.entity(tilemap_entity).add_child(new_tile_entity);
            tile_storage.set(tile_pos, new_tile_entity);
        }
    }
}
