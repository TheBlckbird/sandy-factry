use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;
use infinite::{despawn_outofrange_chunks, spawn_chunks_around_camera};
use rand::{SeedableRng, rngs::StdRng};

mod infinite;

pub const MAP_SIZE: TilemapSize = TilemapSize { x: 1024, y: 1024 };
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 8.0, y: 8.0 };
pub const MAP_TYPE: TilemapType = TilemapType::Square;
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Resource)]
struct GlobalRng(pub StdRng);

impl Default for GlobalRng {
    fn default() -> Self {
        Self(StdRng::from_seed(rand::random()))
    }
}

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Middleground;

#[derive(Component)]
pub struct Chunk;

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

#[derive(Clone, Copy)]
enum MiddlegroundObject {
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

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .init_resource::<GlobalRng>()
            .add_systems(
                Update,
                (spawn_chunks_around_camera, despawn_outofrange_chunks),
            );
    }
}
