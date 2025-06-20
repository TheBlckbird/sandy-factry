use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// TODO: Fix items skipping one in loops
// - add flag to item if it has already moved
// - check before moving

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, Display)]
pub enum ItemType {
    #[strum(to_string = "Coal")]
    Coal,
    #[strum(to_string = "Raw Copper")]
    RawCopper,
    #[strum(to_string = "Raw Iron")]
    RawIron,
    #[strum(to_string = "Copper Ingot")]
    CopperIngot,
    #[strum(to_string = "Iron Ingot")]
    IronIngot,
    #[strum(to_string = "Gear")]
    Gear,
    #[strum(to_string = "Steel")]
    Steel,
    #[strum(to_string = "Wire")]
    Wire,
    #[strum(to_string = "Reinforced Steel")]
    ReinforcedSteel,
    #[strum(to_string = "Electrical Circuit")]
    ElectricalCircuit,
    #[strum(to_string = "Micro Processor")]
    MicroProcessor,
    #[strum(to_string = "Rotor Blade")]
    RotorBlade,
    #[strum(to_string = "Propeller")]
    Propeller,
    #[strum(to_string = "Big Propeller")]
    BigPropeller,
    #[strum(to_string = "Hull")]
    Hull,
    #[strum(to_string = "Motor")]
    Motor,
    #[strum(to_string = "Battery")]
    Battery,
    #[strum(to_string = "Control Module")]
    ControlModule,
    #[strum(to_string = "Helicopter Frame")]
    HelicopterFrame,
    #[strum(to_string = "Engine")]
    Engine,
    #[strum(to_string = "Helicopter")]
    Helicopter,
}

impl ItemType {
    pub fn ends_game(&self) -> bool {
        *self == Self::Helicopter
    }
}

impl From<ItemType> for TileTextureIndex {
    fn from(value: ItemType) -> Self {
        TileTextureIndex(match value {
            ItemType::Coal => 0,
            ItemType::RawCopper => 1,
            ItemType::RawIron => 2,
            ItemType::CopperIngot => 3,
            ItemType::IronIngot => 4,
            ItemType::Gear => 5,
            ItemType::Steel => 6,
            ItemType::Wire => 7,
            ItemType::ReinforcedSteel => 8,
            ItemType::ElectricalCircuit => 9,
            ItemType::MicroProcessor => 10,
            ItemType::RotorBlade => 11,
            ItemType::Propeller => 12,
            ItemType::BigPropeller => 13,
            ItemType::Hull => 14,
            ItemType::Motor => 15,
            ItemType::Battery => 16,
            ItemType::ControlModule => 17,
            ItemType::HelicopterFrame => 18,
            ItemType::Engine => 19,
            ItemType::Helicopter => 20,
        })
    }
}

/// The Item struct stores the item type and whether the item has been moved this tick
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct Item {
    pub has_moved: bool,
    #[deref]
    pub item_type: ItemType,
}

impl From<ItemType> for Item {
    fn from(value: ItemType) -> Self {
        Self {
            has_moved: false,
            item_type: value,
        }
    }
}
