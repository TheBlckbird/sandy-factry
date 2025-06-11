use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use bevy_pkv::PkvStore;
use dyn_clone::clone_box;

use crate::{
    content::machine_types::Machine,
    game_save_types::{GameSave, MachineTiles},
    plugins::{building::foreground_objects::ForegroundObject, world::Seed},
    save_keys::SaveKey,
};

pub fn save_game(
    pkv: &mut PkvStore,
    seed: &Seed,
    machine_tiles: Vec<(&TilePos, &TileTextureIndex, &Machine)>,
    camera_translation: Vec3,
) {
    let mut saved_tiles: MachineTiles = Vec::new();

    for (tile_pos, tile_texture_index, machine) in machine_tiles {
        let foreground_object = ForegroundObject::from(*tile_texture_index);

        saved_tiles.push((
            *tile_pos,
            foreground_object,
            clone_box(&*machine.machine_type),
            machine.input_items.clone(),
            machine.output_items.clone(),
        ));
    }

    let game_save = GameSave::new(saved_tiles, *seed, camera_translation);

    pkv.set(SaveKey::GameSave, &game_save)
        .expect("An error occured while trying to save the game");
}
