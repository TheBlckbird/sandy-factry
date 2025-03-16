use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

#[derive(Resource, Default, PartialEq, Eq)]
enum DrawingMode {
    #[default]
    Idling,
    Drawing,
    Erasing,
}

#[derive(Component)]
struct WasJustPressed(bool);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Game of Life Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            TilemapPlugin,
        ))
        .init_resource::<DrawingMode>()
        .add_systems(Startup, startup)
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, update)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // let mut i = 0;
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        visible: TileVisible(true), //(i % 2 == 0 || i % 7 == 0),
                        ..Default::default()
                    },
                    WasJustPressed(false),
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
            // i += 1;
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    },));
}

fn update(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    tile_query: Query<(Entity, &TilePos, &TileVisible)>,
    mut drawing_mode: ResMut<DrawingMode>,
) {
    if !buttons.pressed(MouseButton::Left) {
        *drawing_mode = DrawingMode::Idling;
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window.single();

    let cursor_position = match window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        Some(position) => position,
        None => return,
    };

    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
        let cursor_in_map_position = {
            let cursor_position = Vec4::from((cursor_position, 0.0, 1.0));
            let cursor_in_map_position = map_transform.compute_matrix().inverse() * cursor_position;
            cursor_in_map_position.xy()
        };

        let clicked_tile_pos =
            match TilePos::from_world_pos(&cursor_in_map_position, map_size, grid_size, map_type) {
                Some(position) => position,
                None => {
                    // println!("whoops");
                    return;
                }
            };

        for (entity, tile_pos, tile_visible) in tile_query.iter_mut() {
            if *tile_pos != clicked_tile_pos {
                continue;
            }

            if *drawing_mode == DrawingMode::Idling {
                if tile_visible.0 {
                    *drawing_mode = DrawingMode::Erasing;
                } else {
                    *drawing_mode = DrawingMode::Drawing;
                }
            }

            let new_tile_visibility = match *drawing_mode {
                DrawingMode::Drawing => true,
                DrawingMode::Erasing => false,
                _ => unreachable!(),
            };

            commands
                .entity(entity)
                .insert(TileVisible(new_tile_visibility));
        }
    }
}
