use bevy::{prelude::*, winit::WinitWindows};
use bevy_ecs_tilemap::prelude::*;
use plugins::{building::BuildingPlugin, simulation::SimulationPlugin};
use winit::window::Icon;

mod buildings;
mod helpers;
mod plugins;

const MAP_SIZE: TilemapSize = TilemapSize { x: 32, y: 32 };
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 8.0, y: 8.0 };
const MAP_TYPE: TilemapType = TilemapType::Square;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Running,
    Paused,
    Background,
}

#[derive(Resource, Default, Clone, Copy)]
enum BackgroundObject {
    Dirt,
    #[default]
    Water,
}

impl From<BackgroundObject> for TileTextureIndex {
    fn from(value: BackgroundObject) -> Self {
        let index = match value {
            BackgroundObject::Dirt => 0,
            BackgroundObject::Water => 1,
        };

        TileTextureIndex(index)
    }
}

#[derive(Component)]
struct BackgroundTile;

#[derive(Component, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn is_orthogonal_to(&self, other: &Direction) -> bool {
        match self {
            Direction::North | Direction::South => {
                matches!(*other, Direction::East | Direction::West)
            }
            Direction::East | Direction::West => {
                matches!(*other, Direction::North | Direction::South)
            }
        }
    }

    pub fn get_opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("The Oil Company"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            TilemapPlugin,
        ))
        .add_plugins((BuildingPlugin, SimulationPlugin))
        .add_systems(
            Startup,
            (
                startup,
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                set_window_icon,
            ),
        )
        .add_systems(Update, helpers::camera::movement)
        .run();
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/app-icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        // this only sets it for Windows and X11
        // The others are set at build time
        window.set_window_icon(Some(icon.clone()));
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::new(0.2, 0.2, 1.0))));

    let background_texture_handle: Handle<Image> = asset_server.load("background_tiles.png");

    let mut background_tile_storage = TileStorage::empty(MAP_SIZE);
    let background_tilemap_entity = commands.spawn_empty().id();

    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(background_tilemap_entity),
                        texture_index: TileTextureIndex(1),
                        ..Default::default()
                    },
                    BackgroundTile,
                ))
                .id();
            background_tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(background_tilemap_entity).insert((
        TilemapBundle {
            grid_size: TILE_SIZE.into(),
            map_type: MAP_TYPE,
            size: MAP_SIZE,
            storage: background_tile_storage,
            texture: TilemapTexture::Single(background_texture_handle.clone()),
            tile_size: TILE_SIZE,
            transform: get_tilemap_center_transform(&MAP_SIZE, &TILE_SIZE.into(), &MAP_TYPE, 0.0),
            ..Default::default()
        },
        BackgroundTile,
    ));
}
