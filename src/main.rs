use bevy::{prelude::*, winit::WinitWindows};
use bevy_ecs_tilemap::prelude::*;
use plugins::{building::BuildingPlugin, simulation::SimulationPlugin, world::WorldPlugin};
use winit::window::Icon;

mod buildings;
mod helpers;
mod plugins;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Running,
    Paused,
    Background,
}

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
        .add_plugins((BuildingPlugin, SimulationPlugin, WorldPlugin))
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

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::new(0.2, 0.2, 1.0))));
}
