use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use generation::generation;
use rand::Rng;

// mod infinite;
mod generation;

// Constants for world generation
pub const MAP_SIZE: TilemapSize = TilemapSize { x: 64, y: 64 };
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 8.0, y: 8.0 };
pub const MAP_TYPE: TilemapType = TilemapType::Square;

#[derive(Resource, Clone, Copy)]
pub struct Seed(u32);

impl Seed {
    fn new() -> Self {
        Self(rand::rng().random())
    }
}

#[derive(Component, Clone, Copy)]
pub struct Background;

#[derive(Component, Clone, Copy)]
pub struct Middleground;

#[allow(unused)]
#[derive(Default, Clone, Copy)]
enum BackgroundObject {
    Dirt,
    Water,
    #[default]
    DefaultTile,
}

impl From<BackgroundObject> for TileTextureIndex {
    fn from(value: BackgroundObject) -> Self {
        let index = match value {
            BackgroundObject::Dirt => 0,
            BackgroundObject::Water => 1,
            BackgroundObject::DefaultTile => 2,
        };

        TileTextureIndex(index)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MiddlegroundObject {
    Coal,
    Copper,
    Iron,
}

impl From<MiddlegroundObject> for TileTextureIndex {
    fn from(value: MiddlegroundObject) -> Self {
        let index = match value {
            MiddlegroundObject::Coal => 0,
            MiddlegroundObject::Copper => 1,
            MiddlegroundObject::Iron => 2,
        };

        TileTextureIndex(index)
    }
}

impl TryFrom<TileTextureIndex> for MiddlegroundObject {
    type Error = ();

    fn try_from(value: TileTextureIndex) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(MiddlegroundObject::Coal),
            1 => Ok(MiddlegroundObject::Copper),
            2 => Ok(MiddlegroundObject::Iron),
            _ => Err(()),
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Seed::new())
            .add_systems(Startup, generation);
    }
}
