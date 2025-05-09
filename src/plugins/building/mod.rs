use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use place_buildings::place_buildings;

use crate::{
    Direction,
    machines::{MachineType, belt::Belt, crafter::Crafter, miner::Miner},
};

use super::world::{MAP_SIZE, MAP_TYPE, TILE_SIZE};

mod place_buildings;

pub struct BuildingPlugin;

#[derive(Resource, Default, Clone, Copy, PartialEq, PartialOrd)]
enum CurrentBuilding {
    #[default]
    Nothing,
    Miner,
    Crafter,
    BeltUp,
    BeltRight,
    BeltDown,
    BeltLeft,
    BeltDownRight,
    BeltLeftDown,
    BeltUpLeft,
    BeltRightUp,
    BeltRightDown,
    BeltDownLeft,
    BeltLeftUp,
    BeltUpRight,
}

impl CurrentBuilding {
    pub fn select_next(&mut self) {
        *self = match self {
            CurrentBuilding::Nothing => CurrentBuilding::Miner,
            CurrentBuilding::Miner => CurrentBuilding::Crafter,
            CurrentBuilding::Crafter => CurrentBuilding::BeltUp,
            CurrentBuilding::BeltUp => CurrentBuilding::BeltRight,
            CurrentBuilding::BeltRight => CurrentBuilding::BeltDown,
            CurrentBuilding::BeltDown => CurrentBuilding::BeltLeft,
            CurrentBuilding::BeltLeft => CurrentBuilding::BeltDownRight,
            CurrentBuilding::BeltDownRight => CurrentBuilding::BeltLeftDown,
            CurrentBuilding::BeltLeftDown => CurrentBuilding::BeltUpLeft,
            CurrentBuilding::BeltUpLeft => CurrentBuilding::BeltRightUp,
            CurrentBuilding::BeltRightUp => CurrentBuilding::BeltRightDown,
            CurrentBuilding::BeltRightDown => CurrentBuilding::BeltDownLeft,
            CurrentBuilding::BeltDownLeft => CurrentBuilding::BeltLeftUp,
            CurrentBuilding::BeltLeftUp => CurrentBuilding::BeltUpRight,
            CurrentBuilding::BeltUpRight => CurrentBuilding::Nothing,
        };
    }

    pub fn select_previous(&mut self) {
        *self = match self {
            CurrentBuilding::Nothing => CurrentBuilding::BeltUpRight,
            CurrentBuilding::Miner => CurrentBuilding::Nothing,
            CurrentBuilding::Crafter => CurrentBuilding::Miner,
            CurrentBuilding::BeltUp => CurrentBuilding::Crafter,
            CurrentBuilding::BeltRight => CurrentBuilding::BeltUp,
            CurrentBuilding::BeltDown => CurrentBuilding::BeltRight,
            CurrentBuilding::BeltLeft => CurrentBuilding::BeltDown,
            CurrentBuilding::BeltDownRight => CurrentBuilding::BeltLeft,
            CurrentBuilding::BeltLeftDown => CurrentBuilding::BeltDownRight,
            CurrentBuilding::BeltUpLeft => CurrentBuilding::BeltLeftDown,
            CurrentBuilding::BeltRightUp => CurrentBuilding::BeltUpLeft,
            CurrentBuilding::BeltRightDown => CurrentBuilding::BeltRightUp,
            CurrentBuilding::BeltDownLeft => CurrentBuilding::BeltRightDown,
            CurrentBuilding::BeltLeftUp => CurrentBuilding::BeltDownLeft,
            CurrentBuilding::BeltUpRight => CurrentBuilding::BeltLeftUp,
        };
    }

    pub fn as_foreground_object(&self) -> ForegroundObject {
        match self {
            CurrentBuilding::Nothing => ForegroundObject::Nothing,
            CurrentBuilding::Miner => ForegroundObject::Miner,
            CurrentBuilding::Crafter => ForegroundObject::Crafter,
            CurrentBuilding::BeltUp => ForegroundObject::BeltUp,
            CurrentBuilding::BeltRight => ForegroundObject::BeltRight,
            CurrentBuilding::BeltDown => ForegroundObject::BeltDown,
            CurrentBuilding::BeltLeft => ForegroundObject::BeltLeft,
            CurrentBuilding::BeltDownRight => ForegroundObject::BeltDownRight,
            CurrentBuilding::BeltLeftDown => ForegroundObject::BeltLeftDown,
            CurrentBuilding::BeltUpLeft => ForegroundObject::BeltUpLeft,
            CurrentBuilding::BeltRightUp => ForegroundObject::BeltRightUp,
            CurrentBuilding::BeltRightDown => ForegroundObject::BeltRightDown,
            CurrentBuilding::BeltDownLeft => ForegroundObject::BeltDownLeft,
            CurrentBuilding::BeltLeftUp => ForegroundObject::BeltLeftUp,
            CurrentBuilding::BeltUpRight => ForegroundObject::BeltUpRight,
        }
    }
}

#[derive(Debug, Resource, Default, Clone, Copy, PartialEq)]
pub enum ForegroundObject {
    #[default]
    Nothing,
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

impl ForegroundObject {
    pub fn into_building_type(self) -> Option<Box<dyn MachineType>> {
        match self {
            ForegroundObject::Nothing => None,
            ForegroundObject::BeltUp
            | ForegroundObject::BeltDown
            | ForegroundObject::BeltLeft
            | ForegroundObject::BeltRight
            | ForegroundObject::BeltDownRight
            | ForegroundObject::BeltLeftDown
            | ForegroundObject::BeltUpLeft
            | ForegroundObject::BeltRightUp
            | ForegroundObject::BeltRightDown
            | ForegroundObject::BeltDownLeft
            | ForegroundObject::BeltLeftUp
            | ForegroundObject::BeltUpRight => Some(Box::new(Belt)),
            ForegroundObject::Crafter => Some(Box::new(Crafter)),
            ForegroundObject::Miner => Some(Box::new(Miner)),
        }
    }

    pub fn get_input_side(&self) -> Option<Direction> {
        match self {
            ForegroundObject::Nothing => None,
            ForegroundObject::BeltUp => Some(Direction::South),
            ForegroundObject::BeltDown => Some(Direction::North),
            ForegroundObject::BeltLeft => Some(Direction::East),
            ForegroundObject::BeltRight => Some(Direction::West),
            ForegroundObject::BeltDownRight => Some(Direction::South),
            ForegroundObject::BeltLeftDown => Some(Direction::West),
            ForegroundObject::BeltUpLeft => Some(Direction::North),
            ForegroundObject::BeltRightUp => Some(Direction::East),
            ForegroundObject::BeltRightDown => Some(Direction::East),
            ForegroundObject::BeltDownLeft => Some(Direction::South),
            ForegroundObject::BeltLeftUp => Some(Direction::West),
            ForegroundObject::BeltUpRight => Some(Direction::North),
            ForegroundObject::Crafter => None,
            ForegroundObject::Miner => Some(Direction::North),
        }
    }

    pub fn get_output_side(&self) -> Option<Direction> {
        match self {
            ForegroundObject::Nothing => None,
            ForegroundObject::BeltUp => Some(Direction::North),
            ForegroundObject::BeltDown => Some(Direction::South),
            ForegroundObject::BeltLeft => Some(Direction::West),
            ForegroundObject::BeltRight => Some(Direction::East),
            ForegroundObject::BeltDownRight => Some(Direction::East),
            ForegroundObject::BeltLeftDown => Some(Direction::South),
            ForegroundObject::BeltUpLeft => Some(Direction::West),
            ForegroundObject::BeltRightUp => Some(Direction::North),
            ForegroundObject::BeltRightDown => Some(Direction::South),
            ForegroundObject::BeltDownLeft => Some(Direction::West),
            ForegroundObject::BeltLeftUp => Some(Direction::North),
            ForegroundObject::BeltUpRight => Some(Direction::East),
            ForegroundObject::Crafter => None,
            ForegroundObject::Miner => Some(Direction::South),
        }
    }

    pub fn should_render_item(&self) -> bool {
        match self {
            ForegroundObject::Nothing | ForegroundObject::Crafter | ForegroundObject::Miner => {
                false
            }
            ForegroundObject::BeltUp
            | ForegroundObject::BeltDown
            | ForegroundObject::BeltLeft
            | ForegroundObject::BeltRight
            | ForegroundObject::BeltDownRight
            | ForegroundObject::BeltLeftDown
            | ForegroundObject::BeltUpLeft
            | ForegroundObject::BeltRightUp
            | ForegroundObject::BeltRightDown
            | ForegroundObject::BeltDownLeft
            | ForegroundObject::BeltLeftUp
            | ForegroundObject::BeltUpRight => true,
        }
    }
}

impl From<TileTextureIndex> for ForegroundObject {
    fn from(value: TileTextureIndex) -> Self {
        match value.0 {
            0 => ForegroundObject::BeltUp,
            1 => ForegroundObject::BeltDown,
            2 => ForegroundObject::BeltRight,
            3 => ForegroundObject::BeltLeft,
            4 => ForegroundObject::BeltDownRight,
            5 => ForegroundObject::BeltLeftDown,
            6 => ForegroundObject::BeltUpLeft,
            7 => ForegroundObject::BeltRightUp,
            8 => ForegroundObject::BeltRightDown,
            9 => ForegroundObject::BeltDownLeft,
            10 => ForegroundObject::BeltLeftUp,
            11 => ForegroundObject::BeltUpRight,
            12 => ForegroundObject::Crafter,
            13 => ForegroundObject::Miner,
            _ => panic!("Can't convert {:?} to a ForegroundObject!", value.0),
        }
    }
}

impl TryFrom<ForegroundObject> for TileTextureIndex {
    type Error = &'static str;

    fn try_from(value: ForegroundObject) -> Result<Self, Self::Error> {
        Ok(TileTextureIndex(match value {
            ForegroundObject::BeltUp => 0,
            ForegroundObject::BeltDown => 1,
            ForegroundObject::BeltRight => 2,
            ForegroundObject::BeltLeft => 3,
            ForegroundObject::BeltDownRight => 4,
            ForegroundObject::BeltLeftDown => 5,
            ForegroundObject::BeltUpLeft => 6,
            ForegroundObject::BeltRightUp => 7,
            ForegroundObject::BeltRightDown => 8,
            ForegroundObject::BeltDownLeft => 9,
            ForegroundObject::BeltLeftUp => 10,
            ForegroundObject::BeltUpRight => 11,
            ForegroundObject::Crafter => 12,
            ForegroundObject::Miner => 13,
            ForegroundObject::Nothing => {
                return Err("Building `Nothing` can't be converted to `ForegroundObject`");
            }
        }))
    }
}

#[allow(unused)]
#[derive(Event)]
pub enum BuildEvent {
    Placed(TilePos, ForegroundObject),
    Deleted(TilePos, ForegroundObject),
}

#[derive(Component)]
pub struct Foreground;

#[derive(Component)]
struct HoverBuilding;

#[derive(Component)]
pub struct BuildingInput(pub Option<Direction>);

#[derive(Component)]
pub struct BuildingOutput(pub Option<Direction>);

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildEvent>()
            .init_resource::<CurrentBuilding>()
            .init_resource::<ForegroundObject>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    select_building,
                    place_buildings,
                    // update_current_foreground_object,
                ),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            anchor: TilemapAnchor::Center,
            ..Default::default()
        },
        Foreground,
    ));
}

fn select_building(mut current_building: ResMut<CurrentBuilding>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyX) {
        current_building.select_next();
    } else if keys.just_pressed(KeyCode::KeyZ) {
        current_building.select_previous();
    } else if keys.just_pressed(KeyCode::KeyQ) {
        *current_building = CurrentBuilding::Nothing;
    }
}

/*
 /‾‾‾‾‾‾\
/<|>  <|>\
|    |   |
\ \____/ /
 \______/
*/
