use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    machines::{InputItems, MachineType, OutputItems},
    plugins::{building::foreground_objects::ForegroundObject, world::Seed},
};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct LoadedGameSave(Option<GameSave>);

#[derive(Serialize, Deserialize)]
pub struct GameSave {
    pub machines: MachineTiles,
    pub seed: Seed,
    pub camera_translation: Vec3,
}

impl GameSave {
    pub fn new(machines: MachineTiles, seed: Seed, camera_translation: Vec3) -> Self {
        Self {
            machines,
            seed,
            camera_translation,
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
