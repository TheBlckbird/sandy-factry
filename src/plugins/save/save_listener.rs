use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use bevy_pkv::PkvStore;
use dyn_clone::clone_box;

use crate::{
    content::machine_types::Machine,
    game_save_types::GameSave,
    plugins::{
        building::foreground_objects::ForegroundObject,
        completion::HasCompletedGame,
        save::{SaveGameMessage, SaveIndicator, SaveIndicatorTimer},
        world::Seed,
    },
    save_keys::SaveKey,
};

pub fn check_save_event(
    mut save_message_reader: MessageReader<SaveGameMessage>,
    mut pkv: ResMut<PkvStore>,
    seed: Res<Seed>,
    tile_query: Query<(&TilePos, &TileTextureIndex, &Machine)>,
    camera: Single<&Transform, With<Camera2d>>,
    has_completed_game: Res<HasCompletedGame>,
    mut save_indicator_visibility: Single<&mut Visibility, With<SaveIndicator>>,
    mut save_indicator_timer: ResMut<SaveIndicatorTimer>,
) {
    let mut show_indicator = false;
    let mut should_save = false;

    for save_game in save_message_reader.read() {
        should_save = true;
        show_indicator |= save_game.show_indicator;
    }

    if !should_save {
        return;
    }

    info!("saving game");

    let mut saved_tiles = Vec::new();

    for (tile_pos, tile_texture_index, machine) in &tile_query {
        let foreground_object = ForegroundObject::from(*tile_texture_index);

        saved_tiles.push((
            *tile_pos,
            foreground_object,
            clone_box(&*machine.machine_type),
            machine.input_items.clone(),
            machine.output_items.clone(),
        ));
    }

    let game_save = GameSave::new(
        saved_tiles,
        *seed,
        camera.into_inner().translation,
        **has_completed_game,
    );

    pkv.set(SaveKey::GameSave, &game_save)
        .expect("An error occured while trying to save the game");

    pkv.set(SaveKey::Version, &env!("CARGO_PKG_VERSION").to_owned())
        .expect("An error occured while trying to save the game version");

    if show_indicator {
        **save_indicator_visibility = Visibility::Visible;
        save_indicator_timer.reset();
    }
}
