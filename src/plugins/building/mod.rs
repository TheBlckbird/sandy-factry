use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    Direction, MAP_SIZE, MAP_TYPE, TILE_SIZE,
    buildings::{BuildingType, Item, belt::Belt, crafter::Crafter, miner::Miner},
};

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

    pub fn into_foreground_object(&self) -> ForegroundObject {
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

#[derive(Resource, Default, Clone, Copy)]
enum ForegroundObject {
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
    pub fn into_tile_texture_index(&self) -> Option<TileTextureIndex> {
        let index = match self {
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
            ForegroundObject::Nothing => return None,
        };

        Some(TileTextureIndex(index))
    }

    pub fn from_tile_texture_index(tile_texture_index: &TileTextureIndex) -> Self {
        match tile_texture_index.0 {
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
            _ => panic!(
                "Can't convert {:?} to a ForegroundObject!",
                tile_texture_index
            ),
        }
    }

    pub fn into_building_type(&self) -> Option<Box<dyn BuildingType>> {
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
            ForegroundObject::Miner => None,
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
            ForegroundObject::Miner => None,
        }
    }
}

#[derive(Event)]
pub enum BuildEvent {
    Placed(TilePos, ForegroundObject),
    Deleted(TilePos, ForegroundObject),
}

#[derive(Component)]
pub struct BuildingComponent {
    pub items: Vec<Item>,
    pub building_type: Box<dyn BuildingType>,
}

impl BuildingComponent {
    pub fn new(items: Vec<Item>, building_type: Box<dyn BuildingType>) -> Self {
        Self {
            items,
            building_type,
        }
    }
}

#[derive(Component)]
struct Foreground;

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
            transform: get_tilemap_center_transform(&MAP_SIZE, &TILE_SIZE.into(), &MAP_TYPE, 1.0),
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
    tile_query: Query<
        (Entity, &TilePos, Option<&HoverBuilding>, &TileTextureIndex),
        With<Foreground>,
    >,
    current_building: Res<CurrentBuilding>,
    mut event_writer: EventWriter<BuildEvent>,
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

    if buttons.pressed(MouseButton::Right) {
        // erasing mode

        for (tile_entity, tile_pos, hover, texture_index) in tile_query.iter() {
            if *tile_pos == mouse_tile_pos || hover.is_some() {
                commands.entity(tile_entity).despawn_recursive();
                tile_storage.remove(&mouse_tile_pos);

                event_writer.send(BuildEvent::Deleted(
                    *tile_pos,
                    ForegroundObject::from_tile_texture_index(texture_index),
                ));
            }
        }

        return;
    }

    // Belt direction

    // let neighbors = Neighbors::get_square_neighboring_positions(&mouse_tile_pos, &map_size, false);
    // let north_neighbor = neighbors.north;
    // let east_neighbor = neighbors.east;
    // let south_neighbor = neighbors.south;
    // let west_neighbor = neighbors.west;

    // let mut connection_options: Vec<(Direction, Direction)> = Vec::new();

    // let find_foreground_object = |searched_tile_pos| {
    //     let mut foreground_object = None;

    //     for (_, tile_pos, _, tile_texture_index) in tile_query.iter() {
    //         if *tile_pos == searched_tile_pos {
    //             foreground_object = Some(ForegroundObject::from_tile_texture_index(
    //                 tile_texture_index,
    //             ));
    //         }
    //     }

    //     foreground_object.unwrap()
    // };

    // let foreground_input_direction = foreground_object.get_input_side();

    // let mut north_neighbor_output_direction = None;
    // let mut east_neighbor_output_direction = None;
    // let mut south_neighbor_output_direction = None;
    // let mut west_neighbor_output_direction = None;

    // if let Some(north_neighbor) = north_neighbor {
    //     let north_foreground_object = find_foreground_object(north_neighbor);
    //     north_neighbor_output_direction = north_foreground_object.get_output_side();
    // }

    // if let Some(east_neighbor) = east_neighbor {
    //     let east_foreground_object = find_foreground_object(east_neighbor);
    //     east_neighbor_output_direction = east_foreground_object.get_output_side();
    // }

    // if let Some(south_neighbor) = south_neighbor {
    //     let south_foreground_object = find_foreground_object(south_neighbor);
    //     south_neighbor_output_direction = south_foreground_object.get_output_side();
    // }

    // if let Some(west_neighbor) = west_neighbor {
    //     let west_foreground_object = find_foreground_object(west_neighbor);
    //     west_neighbor_output_direction = west_foreground_object.get_output_side();
    // }

    // if let Some(north_neighbor_output_direction) = north_neighbor_output_direction {
    //     if let Some(foreground_input_direction) = foreground_input_direction {
    //         if north_neighbor_output_direction
    //         match foreground_input_direction {
    //             Direction::North | Direction::South => {}
    //             Direction::East => todo!(),
    //             Direction::West => todo!(),
    //         }
    //     }
    // }

    // todo!();

    // Belt direction end

    let foreground_object: ForegroundObject = current_building.into_foreground_object();

    let Some(tile_texture_index) = foreground_object.into_tile_texture_index() else {
        return;
    };

    let mut is_other_tile_at_mouse = false;

    for (tile_entity, tile_pos, hover, _) in tile_query.iter() {
        if *tile_pos == mouse_tile_pos && hover.is_none() {
            is_other_tile_at_mouse = true;
        } else if hover.is_some() {
            commands.entity(tile_entity).despawn_recursive();
            tile_storage.remove(tile_pos);
        }
    }

    if is_other_tile_at_mouse {
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        // building mode

        let new_tile_entity = commands
            .spawn((
                TileBundle {
                    position: mouse_tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_texture_index,
                    ..Default::default()
                },
                Foreground,
                BuildingComponent::new(
                    Vec::new(),
                    match foreground_object.into_building_type() {
                        Some(building_type) => building_type,
                        None => return,
                    },
                ),
                BuildingInput(foreground_object.get_input_side()),
                BuildingOutput(foreground_object.get_output_side()),
            ))
            .id();

        event_writer.send(BuildEvent::Placed(
            mouse_tile_pos,
            ForegroundObject::from_tile_texture_index(&tile_texture_index),
        ));

        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    } else {
        // hover mode

        let new_tile_entity = commands
            .spawn((
                TileBundle {
                    position: mouse_tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_texture_index,
                    color: TileColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
                    ..Default::default()
                },
                Foreground,
                HoverBuilding,
            ))
            .id();

        tile_storage.set(&mouse_tile_pos, new_tile_entity);
    }
}

/*
 /‾‾‾‾‾‾\
/<|>  <|>\
|    |   |
\ \____/ /
 \______/
*/
