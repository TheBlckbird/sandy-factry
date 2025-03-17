use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

#[derive(Resource, Default, Clone, Copy)]
enum CurrentBuilding {
    #[default]
    BeltUp,
    BeltDown,
    BeltLeft,
    BeltRight,
    BeltDownRight,
    BeltLeftDown,
    BeltUpLeft,
    BeltRightUp,
    BeltRightDown,
    BeltDownLeft,
    BeltLeftUp,
    BeltUpRight,
    Crafter,
    Miner,
}

impl CurrentBuilding {
    pub fn select_next(&mut self) {
        let next = match self {
            CurrentBuilding::BeltUp => CurrentBuilding::BeltDown,
            CurrentBuilding::BeltDown => CurrentBuilding::BeltLeft,
            CurrentBuilding::BeltLeft => CurrentBuilding::BeltRight,
            CurrentBuilding::BeltRight => CurrentBuilding::BeltDownRight,
            CurrentBuilding::BeltDownRight => CurrentBuilding::BeltLeftDown,
            CurrentBuilding::BeltLeftDown => CurrentBuilding::BeltUpLeft,
            CurrentBuilding::BeltUpLeft => CurrentBuilding::BeltRightUp,
            CurrentBuilding::BeltRightUp => CurrentBuilding::BeltRightDown,
            CurrentBuilding::BeltRightDown => CurrentBuilding::BeltDownLeft,
            CurrentBuilding::BeltDownLeft => CurrentBuilding::BeltLeftUp,
            CurrentBuilding::BeltLeftUp => CurrentBuilding::BeltUpRight,
            CurrentBuilding::BeltUpRight => CurrentBuilding::Crafter,
            CurrentBuilding::Crafter => CurrentBuilding::Miner,
            CurrentBuilding::Miner => CurrentBuilding::BeltUp,
        };

        *self = next;
    }

    pub fn select_previous(&mut self) {
        let next = match self {
            CurrentBuilding::BeltUp => CurrentBuilding::Miner,
            CurrentBuilding::BeltDown => CurrentBuilding::BeltUp,
            CurrentBuilding::BeltLeft => CurrentBuilding::BeltDown,
            CurrentBuilding::BeltRight => CurrentBuilding::BeltLeft,
            CurrentBuilding::BeltDownRight => CurrentBuilding::BeltRight,
            CurrentBuilding::BeltLeftDown => CurrentBuilding::BeltDownRight,
            CurrentBuilding::BeltUpLeft => CurrentBuilding::BeltLeftDown,
            CurrentBuilding::BeltRightUp => CurrentBuilding::BeltUpLeft,
            CurrentBuilding::BeltRightDown => CurrentBuilding::BeltRightUp,
            CurrentBuilding::BeltDownLeft => CurrentBuilding::BeltRightDown,
            CurrentBuilding::BeltLeftUp => CurrentBuilding::BeltDownLeft,
            CurrentBuilding::BeltUpRight => CurrentBuilding::BeltLeftUp,
            CurrentBuilding::Crafter => CurrentBuilding::BeltUpRight,
            CurrentBuilding::Miner => CurrentBuilding::Crafter,
        };

        *self = next;
    }
}

impl From<CurrentBuilding> for TileTextureIndex {
    fn from(value: CurrentBuilding) -> Self {
        let index = match value {
            CurrentBuilding::BeltUp => 0,
            CurrentBuilding::BeltDown => 1,
            CurrentBuilding::BeltLeft => 2,
            CurrentBuilding::BeltRight => 3,
            CurrentBuilding::BeltDownRight => 4,
            CurrentBuilding::BeltLeftDown => 5,
            CurrentBuilding::BeltUpLeft => 6,
            CurrentBuilding::BeltRightUp => 7,
            CurrentBuilding::BeltRightDown => 8,
            CurrentBuilding::BeltDownLeft => 9,
            CurrentBuilding::BeltLeftUp => 10,
            CurrentBuilding::BeltUpRight => 11,
            CurrentBuilding::Crafter => 12,
            CurrentBuilding::Miner => 13,
        };

        TileTextureIndex(index)
    }
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Foreground;

#[derive(Component)]
struct Hover;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bevy Grid World Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            TilemapPlugin,
        ))
        .init_resource::<CurrentBuilding>()
        .add_systems(Startup, startup)
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, (place_buildings, select_building))
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::new(0.2, 0.2, 1.0))));

    let foreground_texture_handle: Handle<Image> = asset_server.load("foreground_tiles.png");
    let background_texture_handle: Handle<Image> = asset_server.load("background_tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };
    let mut background_tile_storage = TileStorage::empty(map_size);
    let foreground_tile_storage = TileStorage::empty(map_size);
    let background_tilemap_entity = commands.spawn_empty().id();
    let foreground_tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(background_tilemap_entity),
                        texture_index: TileTextureIndex(0),
                        ..Default::default()
                    },
                    Background,
                ))
                .id();
            background_tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(background_tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: background_tile_storage,
            texture: TilemapTexture::Single(background_texture_handle.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        Background,
    ));

    commands.entity(foreground_tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: foreground_tile_storage,
            texture: TilemapTexture::Single(foreground_texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
            ..Default::default()
        },
        Foreground,
    ));
}

fn select_building(mut current_building: ResMut<CurrentBuilding>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyC) {
        current_building.select_next();
    } else if keys.just_pressed(KeyCode::KeyX) {
        current_building.select_previous();
    }
}

fn place_buildings(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut tilemap_q: Query<
        (
            Entity,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &Transform,
            &mut TileStorage,
        ),
        With<Foreground>,
    >,
    tile_query: Query<(Entity, &TilePos, Option<&Hover>), With<Foreground>>,
    current_building: Res<CurrentBuilding>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window.single();

    let cursor_position = match window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        Some(position) => position,
        None => return,
    };

    let (tilemap_entity, map_size, grid_size, map_type, map_transform, mut tile_storage) =
        tilemap_q.single_mut();

    let cursor_in_map_position = {
        let cursor_position = Vec4::from((cursor_position, 0.0, 1.0));
        let cursor_in_map_position = map_transform.compute_matrix().inverse() * cursor_position;
        cursor_in_map_position.xy()
    };

    let mouse_tile_pos =
        match TilePos::from_world_pos(&cursor_in_map_position, map_size, grid_size, map_type) {
            Some(position) => position,
            None => return,
        };

    if buttons.pressed(MouseButton::Left) {
        // building mode

        for (tile_entity, tile_pos, hover) in tile_query.iter() {
            if hover.is_some() {
                commands.entity(tile_entity).despawn_recursive();
                tile_storage.remove(&tile_pos);
            }
        }

        let new_tile_entity = commands
            .spawn((
                TileBundle {
                    position: mouse_tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex::from(*current_building),
                    ..Default::default()
                },
                Foreground,
            ))
            .id();

        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    } else if buttons.pressed(MouseButton::Right) {
        // erasing mode

        for (tile_entity, tile_pos, hover) in tile_query.iter() {
            if *tile_pos == mouse_tile_pos || hover.is_some() {
                commands.entity(tile_entity).despawn_recursive();
                tile_storage.remove(&mouse_tile_pos);
            }
        }
    } else {
        // hover mode

        let mut is_tile_at_mouse = false;

        for (tile_entity, tile_pos, hover) in tile_query.iter() {
            if *tile_pos == mouse_tile_pos && hover.is_none() {
                is_tile_at_mouse = true;
            } else if hover.is_some() {
                commands.entity(tile_entity).despawn_recursive();
                tile_storage.remove(&tile_pos);
            }
        }

        if !is_tile_at_mouse {
            let new_tile_entity = commands
                .spawn((
                    TileBundle {
                        position: mouse_tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex::from(*current_building),
                        color: TileColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
                        ..Default::default()
                    },
                    Foreground,
                    Hover,
                ))
                .id();

            tile_storage.set(&mouse_tile_pos, new_tile_entity);
        }
    }
}
