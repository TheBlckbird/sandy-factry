use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use foreground_objects::CurrentMachine;
use load_game_save::load_game_save;
use place_buildings::place_buildings;
use serde::{Deserialize, Serialize};

use crate::{
    Direction,
    plugins::{
        RenderLayer,
        interaction::can_interact_with_world,
        menu::{GameState, game_menus::GameMenuState},
        world::{MAP_SIZE, MAP_TYPE, TILE_SIZE},
    },
};

pub mod foreground_objects;
mod load_game_save;
mod place_buildings;

// MARK: Plugin

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildEvent>()
            .add_systems(OnEnter(GameState::Game), (setup, load_game_save).chain())
            .add_systems(
                Update,
                (select_building, place_buildings).run_if(can_interact_with_world),
            )
            .add_systems(OnExit(GameMenuState::Hidden), deselect_current_building)
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

// MARK: Events

#[allow(unused)]
#[derive(Event)]
pub enum BuildEvent {
    Placed(TilePos, foreground_objects::ForegroundObject),
    Deleted(TilePos, foreground_objects::ForegroundObject),
}

// MARK: Components

#[derive(Component)]
pub struct Foreground;

#[derive(Component)]
struct HoverBuilding;

#[derive(Component, Serialize, Deserialize, Deref, DerefMut)]
pub struct BuildingInput(pub Option<Vec<Direction>>);

#[derive(Component, Deref, DerefMut)]
pub struct BuildingOutput(pub Option<Vec<Direction>>);

// MARK: Systems

/// Initialize everything for building
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add resource
    commands.init_resource::<CurrentMachine>();

    let foreground_texture_handle: Handle<Image> = asset_server.load("foreground_tiles.png");

    let foreground_tile_storage = TileStorage::empty(MAP_SIZE);
    let foreground_tilemap_entity = commands.spawn_empty().id();

    // Spawn tilemap
    commands.entity(foreground_tilemap_entity).insert((
        TilemapBundle {
            grid_size: TILE_SIZE.into(),
            map_type: MAP_TYPE,
            size: MAP_SIZE,
            storage: foreground_tile_storage,
            texture: TilemapTexture::Single(foreground_texture_handle),
            tile_size: TILE_SIZE,
            transform: Transform::from_xyz(0.0, 0.0, RenderLayer::Foreground.into()),
            anchor: TilemapAnchor::Center,
            ..Default::default()
        },
        Foreground,
    ));
}

/// Check for keyboard inputs to select or rotate buildings
fn select_building(mut current_building: ResMut<CurrentMachine>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyX) {
        current_building.select_next_machine();
    } else if keys.just_pressed(KeyCode::KeyZ) {
        current_building.select_prev_machine();
    } else if keys.just_pressed(KeyCode::KeyR) {
        current_building.select_next_variant();
    } else if keys.just_pressed(KeyCode::KeyF) {
        current_building.select_prev_variant();
    } else if keys.just_pressed(KeyCode::KeyQ) {
        current_building.deselect();
    }

    let mut n = None;

    if keys.just_pressed(KeyCode::Digit1) {
        n = Some(1);
    } else if keys.just_pressed(KeyCode::Digit2) {
        n = Some(2);
    } else if keys.just_pressed(KeyCode::Digit3) {
        n = Some(3);
    } else if keys.just_pressed(KeyCode::Digit4) {
        n = Some(4);
    } else if keys.just_pressed(KeyCode::Digit5) {
        n = Some(5);
    } else if keys.just_pressed(KeyCode::Digit6) {
        n = Some(6);
    } else if keys.just_pressed(KeyCode::Digit7) {
        n = Some(7);
    } else if keys.just_pressed(KeyCode::Digit8) {
        n = Some(8);
    } else if keys.just_pressed(KeyCode::Digit9) {
        n = Some(9);
    } else if keys.just_pressed(KeyCode::Digit0) {
        n = Some(10);
    }

    if let Some(n) = n {
        current_building.select_nth_machine(n);
    }
}

/// Deselects the current building
fn deselect_current_building(mut current_building: ResMut<CurrentMachine>) {
    current_building.deselect();
}

/// Remove everything building related, specifically the tilemap and resource
fn cleanup(
    mut commands: Commands,
    foreground_tilemap: Single<Entity, (With<TileStorage>, With<Foreground>)>,
) {
    commands.entity(foreground_tilemap.entity()).despawn();
    commands.remove_resource::<CurrentMachine>();
}

/*
 /‾‾‾‾‾‾\
/<|>  <|>\
|    |   |
\ \____/ /
 \______/
*/
