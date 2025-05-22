use crate::machines::{
    Item, MachineType, Side, belt::Belt, combiner::Combiner, crafter::Crafter, miner::Miner,
};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::super::crafting::recipe_types::CrafterRecipe;

#[derive(Debug, Resource, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    CombinerUpLeft,
    CombinerLeftDown,
    CombinerDownRight,
    CombinerRightUp,
    CombinerDownLeft,
    CombinerLeftUp,
    CombinerUpRight,
    CombinerRightDown,
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
            ForegroundObject::Crafter => Some(Box::new(Crafter::new(Some(CrafterRecipe::new(
                HashMap::from([(Item::Coal, 1), (Item::RawCopper, 2)]),
                Item::CopperIngot,
                1,
                1,
            ))))),
            ForegroundObject::Miner => Some(Box::new(Miner)),
            ForegroundObject::CombinerUpLeft => {
                Some(Box::new(Combiner::new([Side::North, Side::West])))
            }
            ForegroundObject::CombinerLeftDown => {
                Some(Box::new(Combiner::new([Side::West, Side::South])))
            }
            ForegroundObject::CombinerDownRight => {
                Some(Box::new(Combiner::new([Side::South, Side::East])))
            }
            ForegroundObject::CombinerRightUp => {
                Some(Box::new(Combiner::new([Side::East, Side::North])))
            }
            ForegroundObject::CombinerDownLeft => {
                Some(Box::new(Combiner::new([Side::South, Side::West])))
            }
            ForegroundObject::CombinerLeftUp => {
                Some(Box::new(Combiner::new([Side::West, Side::North])))
            }
            ForegroundObject::CombinerUpRight => {
                Some(Box::new(Combiner::new([Side::North, Side::East])))
            }
            ForegroundObject::CombinerRightDown => {
                Some(Box::new(Combiner::new([Side::East, Side::South])))
            }
        }
    }

    pub fn get_input_sides(&self) -> Option<Vec<Side>> {
        match self {
            ForegroundObject::Nothing => None,
            ForegroundObject::BeltUp => Some(vec![Side::South]),
            ForegroundObject::BeltDown => Some(vec![Side::North]),
            ForegroundObject::BeltLeft => Some(vec![Side::East]),
            ForegroundObject::BeltRight => Some(vec![Side::West]),
            ForegroundObject::BeltDownRight => Some(vec![Side::South]),
            ForegroundObject::BeltLeftDown => Some(vec![Side::West]),
            ForegroundObject::BeltUpLeft => Some(vec![Side::North]),
            ForegroundObject::BeltRightUp => Some(vec![Side::East]),
            ForegroundObject::BeltRightDown => Some(vec![Side::East]),
            ForegroundObject::BeltDownLeft => Some(vec![Side::South]),
            ForegroundObject::BeltLeftUp => Some(vec![Side::West]),
            ForegroundObject::BeltUpRight => Some(vec![Side::North]),
            ForegroundObject::Crafter => Some(vec![Side::North, Side::West]),
            ForegroundObject::Miner => Some(vec![Side::North]),
            ForegroundObject::CombinerUpLeft => Some(vec![Side::North, Side::West]),
            ForegroundObject::CombinerLeftDown => Some(vec![Side::West, Side::South]),
            ForegroundObject::CombinerDownRight => Some(vec![Side::South, Side::East]),
            ForegroundObject::CombinerRightUp => Some(vec![Side::East, Side::North]),
            ForegroundObject::CombinerDownLeft => Some(vec![Side::South, Side::West]),
            ForegroundObject::CombinerLeftUp => Some(vec![Side::West, Side::North]),
            ForegroundObject::CombinerUpRight => Some(vec![Side::North, Side::East]),
            ForegroundObject::CombinerRightDown => Some(vec![Side::East, Side::South]),
        }
    }

    pub fn get_output_side(&self) -> Option<Side> {
        match self {
            ForegroundObject::Nothing => None,
            ForegroundObject::BeltUp => Some(Side::North),
            ForegroundObject::BeltDown => Some(Side::South),
            ForegroundObject::BeltLeft => Some(Side::West),
            ForegroundObject::BeltRight => Some(Side::East),
            ForegroundObject::BeltDownRight => Some(Side::East),
            ForegroundObject::BeltLeftDown => Some(Side::South),
            ForegroundObject::BeltUpLeft => Some(Side::West),
            ForegroundObject::BeltRightUp => Some(Side::North),
            ForegroundObject::BeltRightDown => Some(Side::South),
            ForegroundObject::BeltDownLeft => Some(Side::West),
            ForegroundObject::BeltLeftUp => Some(Side::North),
            ForegroundObject::BeltUpRight => Some(Side::East),
            ForegroundObject::Crafter => Some(Side::South),
            ForegroundObject::Miner => Some(Side::South),
            ForegroundObject::CombinerUpLeft => Some(Side::South),
            ForegroundObject::CombinerLeftDown => Some(Side::East),
            ForegroundObject::CombinerDownRight => Some(Side::North),
            ForegroundObject::CombinerRightUp => Some(Side::West),
            ForegroundObject::CombinerDownLeft => Some(Side::North),
            ForegroundObject::CombinerLeftUp => Some(Side::East),
            ForegroundObject::CombinerUpRight => Some(Side::South),
            ForegroundObject::CombinerRightDown => Some(Side::West),
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
            | ForegroundObject::BeltUpRight
            | ForegroundObject::CombinerUpLeft
            | ForegroundObject::CombinerLeftDown
            | ForegroundObject::CombinerDownRight
            | ForegroundObject::CombinerRightUp
            | ForegroundObject::CombinerDownLeft
            | ForegroundObject::CombinerLeftUp
            | ForegroundObject::CombinerUpRight
            | ForegroundObject::CombinerRightDown => true,
        }
    }

    pub fn select_previous(&mut self) {
        *self = match self {
            ForegroundObject::Nothing => ForegroundObject::CombinerUpRight,
            ForegroundObject::BeltUp => ForegroundObject::Nothing,
            ForegroundObject::BeltDown => ForegroundObject::BeltUp,
            ForegroundObject::BeltLeft => ForegroundObject::BeltDown,
            ForegroundObject::BeltRight => ForegroundObject::BeltLeft,
            ForegroundObject::BeltDownRight => ForegroundObject::BeltRight,
            ForegroundObject::BeltLeftDown => ForegroundObject::BeltDownRight,
            ForegroundObject::BeltUpLeft => ForegroundObject::BeltLeftDown,
            ForegroundObject::BeltRightUp => ForegroundObject::BeltUpLeft,
            ForegroundObject::BeltRightDown => ForegroundObject::BeltRightUp,
            ForegroundObject::BeltDownLeft => ForegroundObject::BeltRightDown,
            ForegroundObject::BeltLeftUp => ForegroundObject::BeltDownLeft,
            ForegroundObject::BeltUpRight => ForegroundObject::BeltLeftUp,
            ForegroundObject::Crafter => ForegroundObject::BeltUpRight,
            ForegroundObject::Miner => ForegroundObject::Crafter,
            ForegroundObject::CombinerUpLeft => ForegroundObject::Miner,
            ForegroundObject::CombinerLeftDown => ForegroundObject::CombinerUpLeft,
            ForegroundObject::CombinerDownRight => ForegroundObject::CombinerLeftDown,
            ForegroundObject::CombinerRightUp => ForegroundObject::CombinerDownRight,
            ForegroundObject::CombinerDownLeft => ForegroundObject::CombinerRightUp,
            ForegroundObject::CombinerLeftUp => ForegroundObject::CombinerDownLeft,
            ForegroundObject::CombinerUpRight => ForegroundObject::CombinerLeftUp,
            ForegroundObject::CombinerRightDown => ForegroundObject::CombinerUpRight,
        };
    }

    pub fn select_next(&mut self) {
        *self = match self {
            ForegroundObject::Nothing => ForegroundObject::BeltUp,
            ForegroundObject::BeltUp => ForegroundObject::BeltDown,
            ForegroundObject::BeltDown => ForegroundObject::BeltLeft,
            ForegroundObject::BeltLeft => ForegroundObject::BeltRight,
            ForegroundObject::BeltRight => ForegroundObject::BeltDownRight,
            ForegroundObject::BeltDownRight => ForegroundObject::BeltLeftDown,
            ForegroundObject::BeltLeftDown => ForegroundObject::BeltUpLeft,
            ForegroundObject::BeltUpLeft => ForegroundObject::BeltRightUp,
            ForegroundObject::BeltRightUp => ForegroundObject::BeltRightDown,
            ForegroundObject::BeltRightDown => ForegroundObject::BeltDownLeft,
            ForegroundObject::BeltDownLeft => ForegroundObject::BeltLeftUp,
            ForegroundObject::BeltLeftUp => ForegroundObject::BeltUpRight,
            ForegroundObject::BeltUpRight => ForegroundObject::Crafter,
            ForegroundObject::Crafter => ForegroundObject::Miner,
            ForegroundObject::Miner => ForegroundObject::CombinerUpLeft,
            ForegroundObject::CombinerUpLeft => ForegroundObject::CombinerLeftDown,
            ForegroundObject::CombinerLeftDown => ForegroundObject::CombinerDownRight,
            ForegroundObject::CombinerDownRight => ForegroundObject::CombinerRightUp,
            ForegroundObject::CombinerRightUp => ForegroundObject::CombinerDownLeft,
            ForegroundObject::CombinerDownLeft => ForegroundObject::CombinerLeftUp,
            ForegroundObject::CombinerLeftUp => ForegroundObject::CombinerUpRight,
            ForegroundObject::CombinerUpRight => ForegroundObject::CombinerRightDown,
            ForegroundObject::CombinerRightDown => ForegroundObject::Nothing,
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
            14 => ForegroundObject::CombinerUpLeft,
            15 => ForegroundObject::CombinerLeftDown,
            16 => ForegroundObject::CombinerDownRight,
            17 => ForegroundObject::CombinerRightUp,
            18 => ForegroundObject::CombinerDownLeft,
            19 => ForegroundObject::CombinerLeftUp,
            20 => ForegroundObject::CombinerUpRight,
            21 => ForegroundObject::CombinerRightDown,
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
            ForegroundObject::CombinerUpLeft => 14,
            ForegroundObject::CombinerLeftDown => 15,
            ForegroundObject::CombinerDownRight => 16,
            ForegroundObject::CombinerRightUp => 17,
            ForegroundObject::CombinerDownLeft => 18,
            ForegroundObject::CombinerLeftUp => 19,
            ForegroundObject::CombinerUpRight => 20,
            ForegroundObject::CombinerRightDown => 21,
            ForegroundObject::Nothing => {
                return Err("Building `Nothing` can't be converted to `ForegroundObject`");
            }
        }))
    }
}
