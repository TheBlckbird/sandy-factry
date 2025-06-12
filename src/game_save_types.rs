use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    content::machine_types::{InputItems, MachineType, OutputItems},
    plugins::{building::foreground_objects::ForegroundObject, world::Seed},
};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct LoadedGameSave(Option<GameSave>);

/// All the information saved to disk after closing the game are in this struct
#[derive(Serialize, Deserialize)]
pub struct GameSave {
    pub machines: MachineTiles,
    pub seed: Seed,
    pub camera_translation: Vec3,
    pub has_completed_game: bool,
}

impl GameSave {
    pub fn new(
        machines: MachineTiles,
        seed: Seed,
        camera_translation: Vec3,
        has_completed_game: bool,
    ) -> Self {
        Self {
            machines,
            seed,
            camera_translation,
            has_completed_game,
        }
    }
}

pub type MachineTiles = Vec<(
    TilePos,
    ForegroundObject,
    Box<dyn MachineType>,
    InputItems,
    OutputItems,
)>;
