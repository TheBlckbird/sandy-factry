use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_pkv::{GetError, PkvStore};

use crate::{
    machines::Machine,
    plugins::building::{BuildEvent, BuildingInput, BuildingOutput, Foreground},
    save_keys::{GameSave, SaveKey},
};

pub fn load_game_save(
    mut commands: Commands,
    pkv: Res<PkvStore>,
    foreground_tilemap: Single<(Entity, &mut TileStorage), With<Foreground>>,
    mut event_writer: EventWriter<BuildEvent>,
) {
    let game_save: Result<GameSave, GetError> = pkv.get(SaveKey::GameSave);
    let (tilemap_entity, mut tile_storage) = foreground_tilemap.into_inner();

    match game_save {
        Ok(game_save) => {
            for (tile_pos, foreground_object, input_items, output_items) in game_save.machines {
                let new_tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: foreground_object
                                .try_into()
                                .expect("`Nothing` tile found in game save!"),
                            ..Default::default()
                        },
                        Foreground,
                        Machine::new(
                            match foreground_object.into_building_type() {
                                Some(building_type) => building_type,
                                None => return,
                            },
                            input_items,
                            output_items,
                        ),
                        BuildingInput(foreground_object.get_input_sides()),
                        BuildingOutput(foreground_object.get_output_side()),
                    ))
                    .id();

                event_writer.write(BuildEvent::Placed(tile_pos, foreground_object));

                commands.entity(tilemap_entity).add_child(new_tile_entity);
                tile_storage.set(&tile_pos, new_tile_entity);
            }
        }
        Err(GetError::NotFound) => {}
        _ => panic!(
            "An Error occured while trying to load the save state\nTry tdo delete the save file (/Users/username/Library/Application Support/louisweigel.sandy-factry/bevy_pkv.redb) on MacOS.\nThis WILL delete all your save data!"
        ),
    }
}
