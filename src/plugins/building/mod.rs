use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use load_game_save::load_game_save;
use place_buildings::place_buildings;
use serde::{Deserialize, Serialize};

use crate::Direction;

use super::{
    TilemapLayer,
    menu::GameState,
    world::{MAP_SIZE, MAP_TYPE, TILE_SIZE},
};

pub mod foreground_objects;
mod load_game_save;
mod place_buildings;

#[allow(unused)]
#[derive(Event)]
pub enum BuildEvent {
    Placed(TilePos, foreground_objects::ForegroundObject),
    Deleted(TilePos, foreground_objects::ForegroundObject),
}

#[derive(Component)]
pub struct Foreground;

#[derive(Component)]
struct HoverBuilding;

#[derive(Component, Serialize, Deserialize)]
pub struct BuildingInput(pub Option<Vec<Direction>>);

#[derive(Component)]
pub struct BuildingOutput(pub Option<Direction>);

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildEvent>()
            .add_systems(OnEnter(GameState::Game), (setup, load_game_save).chain())
            .add_systems(
                Update,
                (
                    select_building,
                    place_buildings,
                    // update_current_foreground_object,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.init_resource::<foreground_objects::ForegroundObject>();

    let foreground_texture_handle: Handle<Image> = asset_server.load("foreground_tiles.png");

    let foreground_tile_storage = TileStorage::empty(MAP_SIZE);
    let foreground_tilemap_entity = commands.spawn_empty().id();

    commands.entity(foreground_tilemap_entity).insert((
        TilemapBundle {
            grid_size: TILE_SIZE.into(),
            map_type: MAP_TYPE,
            size: MAP_SIZE,
            storage: foreground_tile_storage,
            texture: TilemapTexture::Single(foreground_texture_handle),
            tile_size: TILE_SIZE,
            transform: Transform::from_xyz(0.0, 0.0, TilemapLayer::Foreground.into()),
            anchor: TilemapAnchor::Center,
            ..Default::default()
        },
        Foreground,
    ));
}

fn select_building(
    mut current_building: ResMut<foreground_objects::ForegroundObject>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyX) {
        current_building.select_next();
    } else if keys.just_pressed(KeyCode::KeyZ) {
        current_building.select_previous();
    } else if keys.just_pressed(KeyCode::KeyQ) {
        *current_building = foreground_objects::ForegroundObject::Nothing;
    }
}

fn cleanup(
    mut commands: Commands,
    foreground_tilemap: Single<Entity, (With<TileStorage>, With<Foreground>)>,
) {
    commands.entity(foreground_tilemap.entity()).despawn();
    commands.remove_resource::<foreground_objects::ForegroundObject>();
}

/*
 /‾‾‾‾‾‾\
/<|>  <|>\
|    |   |
\ \____/ /
 \______/
*/
