#![feature(let_chains)]

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_ecs_tilemap::prelude::*;
use helpers::tilemap::get_mouse_tilepos;
use plugins::{
    building::{BuildingPlugin, Foreground},
    hud::HudPlugin,
    simulation::SimulationPlugin,
    world::WorldPlugin,
};
use winit::window::Icon;

mod helpers;
mod machines;
mod plugins;

#[derive(Resource, Default)]
pub struct MouseCoordinates {
    pub x: u32,
    pub y: u32,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Running,
    Paused,
    Background,
}

#[derive(Component, PartialEq)]
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
        .add_plugins((BuildingPlugin, SimulationPlugin, WorldPlugin, HudPlugin))
        .init_resource::<MouseCoordinates>()
        .add_systems(
            Startup,
            (
                startup,
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                set_window_icon,
            ),
        )
        .add_systems(Update, (helpers::camera::movement, update_mouse_coords))
        .run();
}

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
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        // this only sets it for Windows and X11
        // The others are set at build time
        window.set_window_icon(Some(icon.clone()));
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::new(0.2, 0.2, 1.0))));
}

fn update_mouse_coords(
    mut mouse_coordinates: ResMut<MouseCoordinates>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    map_transform_q: Query<
        (&Transform, &TilemapSize, &TilemapGridSize, &TilemapType),
        With<Foreground>,
    >,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();
    let (map_transform, map_size, grid_size, map_type) = map_transform_q.single();

    let Some(new_mouse_coords) = get_mouse_tilepos(
        camera,
        window,
        camera_transform,
        map_transform,
        map_size,
        grid_size,
        map_type,
    ) else {
        return;
    };

    mouse_coordinates.x = new_mouse_coords.x as u32;
    mouse_coordinates.y = new_mouse_coords.y as u32;
}
