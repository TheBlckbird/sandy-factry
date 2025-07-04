#![feature(if_let_guard)] // [TODO] This can probably be removed in a few days
#![windows_subsystem = "windows"]

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_ecs_tilemap::prelude::*;
use bevy_pkv::PkvStore;
use plugins::{
    building::{BuildingPlugin, Foreground},
    crafting::CraftingPlugin,
    debug_camera::DebugCameraPlugin,
    hud::HudPlugin,
    menu::{GameState, MenuPlugin},
    rendering::RenderingPlugin,
    simulation::SimulationPlugin,
    world::WorldPlugin,
};
use sandy_factry_helpers::tilemap::{TilemapSettingsBorrowed, get_mouse_tilepos};
use serde::{Deserialize, Serialize};
use winit::window::Icon;

use crate::plugins::{
    auto_save::AutoSavePlugin, completion::CompletionPlugin, interaction::MachineInteractionPlugin,
};

mod content;
mod game_save_types;
mod plugins;
mod save_game;
mod save_keys;

#[derive(Resource, Default)]
pub struct MouseCoordinates {
    pub x: u32,
    pub y: u32,
}

impl MouseCoordinates {
    pub fn as_tile_pos(&self) -> TilePos {
        TilePos::new(self.x, self.y)
    }
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Direction {
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

    pub fn as_ivec2(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, 1),
            Direction::East => IVec2::new(1, 0),
            Direction::South => IVec2::new(0, -1),
            Direction::West => IVec2::new(-1, 0),
        }
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Sandy Fact'ry"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            TilemapPlugin,
        ))
        .add_plugins((
            BuildingPlugin,
            SimulationPlugin,
            WorldPlugin,
            HudPlugin,
            RenderingPlugin,
            DebugCameraPlugin,
            CraftingPlugin,
            MenuPlugin,
            MachineInteractionPlugin,
            CompletionPlugin,
            AutoSavePlugin,
        ))
        .insert_resource(PkvStore::new("com.louisweigel", "sandy-factry"))
        .init_resource::<MouseCoordinates>()
        .insert_resource(ClearColor(Color::hsl(194.0, 0.71, 0.37)))
        .add_systems(
            PreStartup,
            (
                startup,
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                set_window_icon,
            ),
        )
        .add_systems(
            PreUpdate,
            update_mouse_coords.run_if(in_state(GameState::Game)),
        )
        .run()
}

/// Set the window icon on windows and X11
///
/// MacOS and Wayland get the icons from the desktop entry
#[allow(unused)]
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/app-icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("The icon should adhere to the following rules: The length of `rgba` must be divisible by 4, and `width * height` must equal `rgba.len() / 4`.");

    // do it for all windows
    for window in windows.windows.values() {
        // this only sets it for Windows and X11
        // The others are set at build time
        window.set_window_icon(Some(icon.clone()));
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::new(0.2, 0.2, 1.0)),
        Msaa::Off,
    ));
}

/// Update the resource holding the mouse coordinates
fn update_mouse_coords(
    mut mouse_coordinates: ResMut<MouseCoordinates>,
    camera_q: Single<(&Camera, &GlobalTransform)>,
    window_q: Single<&Window, With<PrimaryWindow>>,
    map_transform_q: Single<
        (
            &Transform,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TilemapTileSize,
            &TilemapAnchor,
        ),
        With<Foreground>,
    >,
) {
    let (camera, camera_transform) = camera_q.into_inner();
    let window = window_q.into_inner();
    let (map_transform, map_size, grid_size, map_type, tile_size, anchor) =
        map_transform_q.into_inner();

    let Some(new_mouse_coords) = get_mouse_tilepos(
        camera,
        window,
        camera_transform,
        map_transform,
        TilemapSettingsBorrowed::new(map_size, tile_size, map_type, grid_size),
        anchor,
    ) else {
        return;
    };

    mouse_coordinates.x = new_mouse_coords.x;
    mouse_coordinates.y = new_mouse_coords.y;
}
