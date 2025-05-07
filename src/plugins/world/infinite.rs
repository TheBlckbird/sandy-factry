use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{Rng, rngs::StdRng};

use super::{
    Background, BackgroundObject, CHUNK_SIZE, Chunk, ChunkManager, ChunkPos, GlobalRng,
    Middleground, MiddlegroundObject, RENDER_CHUNK_SIZE, TILE_SIZE,
};

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut rng: ResMut<GlobalRng>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());

        for y in (camera_chunk_pos.y - (CHUNK_SIZE.y / 2) as i32)
            ..(camera_chunk_pos.y + (CHUNK_SIZE.y / 2) as i32)
        {
            for x in (camera_chunk_pos.x - (CHUNK_SIZE.x / 2) as i32)
                ..(camera_chunk_pos.x + (CHUNK_SIZE.x / 2) as i32)
            {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y), &mut rng.0);
                }
            }
        }
    }
}

pub fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform), With<Chunk>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);

            if distance > 320.0 {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn();
            }
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    rng: &mut StdRng,
) {
    let translation = Vec2::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x + TILE_SIZE.x / 2.0,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y - TILE_SIZE.y / 2.0,
    );

    let background_tilemap_entity = commands.spawn_empty().id();
    let mut background_tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    // Spawn the tiles for the background tilemap
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(background_tilemap_entity),
                        texture_index: BackgroundObject::DefaultTile.into(),
                        ..Default::default()
                    },
                    Background,
                    ChunkPos(IVec2 {
                        x: x as i32,
                        y: y as i32,
                    }),
                ))
                .id();
            commands
                .entity(background_tilemap_entity)
                .add_child(tile_entity);
            background_tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let background_texture_handle: Handle<Image> = asset_server.load("background_tiles.png");

    // Spawn the background tilemap
    commands.entity(background_tilemap_entity).insert((
        make_tilemap_bundle(
            Transform::from_translation(translation.extend(0.0)),
            background_texture_handle,
            background_tile_storage,
        ),
        Background,
        Chunk,
    ));

    let middleground_tilemap_entity = commands.spawn_empty().id();
    let mut middleground_tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    // Spawn the tiles for the middleground tilemap
    let maybe_resource_type = match rng.random_range(0..20) {
        0 => Some(MiddlegroundObject::Coal),
        1 => Some(MiddlegroundObject::Iron),
        2 => Some(MiddlegroundObject::Copper),
        _ => None,
    };

    if let Some(resource_type) = maybe_resource_type {
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(middleground_tilemap_entity),
                            texture_index: resource_type.into(),
                            ..Default::default()
                        },
                        Middleground,
                        ChunkPos(IVec2 {
                            x: x as i32,
                            y: y as i32,
                        }),
                    ))
                    .id();
                commands
                    .entity(middleground_tilemap_entity)
                    .add_child(tile_entity);
                middleground_tile_storage.set(&tile_pos, tile_entity);
            }
        }

        let middleground_texture_handle = asset_server.load("middleground_tiles.png");

        // Spawn the middleground tilemap
        commands.entity(middleground_tilemap_entity).insert((
            make_tilemap_bundle(
                Transform::from_translation(translation.extend(1.0)),
                middleground_texture_handle,
                middleground_tile_storage,
            ),
            Middleground,
            Chunk,
        ));
    }
}

fn make_tilemap_bundle(
    transform: Transform,
    texture_handle: Handle<Image>,
    tile_storage: TileStorage,
) -> TilemapBundle {
    TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform,
        render_settings: TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}
