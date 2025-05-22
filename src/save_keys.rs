use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

use crate::{
    machines::{InputItems, OutputItems},
    plugins::{building::ForegroundObject, world::Seed},
};

pub type MachineTiles = Vec<(TilePos, ForegroundObject, InputItems, OutputItems)>;

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

#[derive(AsRefStr)]
pub enum SaveKey {
    GameSave,
}
