use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use bevy_pkv::PkvStore;

use crate::{
    content::machine_types::Machine,
    plugins::{
        auto_save::{AutoSaveTimer, SaveIndicator, SaveIndicatorTimer},
        completion::HasCompletedGame,
        world::Seed,
    },
    save_game::save_game,
};

pub fn check_auto_save(
    mut auto_save_timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
    mut pkv: ResMut<PkvStore>,
    seed: Res<Seed>,
    tile_query: Query<(&TilePos, &TileTextureIndex, &Machine)>,
    camera: Single<&Transform, With<Camera2d>>,
    has_completed_game: Res<HasCompletedGame>,
    mut save_indicator_visibility: Single<&mut Visibility, With<SaveIndicator>>,
    mut save_indicator_timer: ResMut<SaveIndicatorTimer>,
) {
    if auto_save_timer.tick(time.delta()).just_finished() {
        info!("saving game");

        save_game(
            &mut pkv,
            &seed,
            tile_query.iter().collect(),
            camera.into_inner().translation,
            **has_completed_game,
        );

        **save_indicator_visibility = Visibility::Visible;
        save_indicator_timer.reset();
    }
}
