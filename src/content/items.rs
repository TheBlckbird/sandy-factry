use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, Display)]
pub enum Item {
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

impl Item {
    pub fn ends_game(&self) -> bool {
        *self == Self::Helicopter
    }
}

impl From<Item> for TileTextureIndex {
    fn from(value: Item) -> Self {
        TileTextureIndex(match value {
            Item::Coal => 0,
            Item::RawCopper => 1,
            Item::RawIron => 2,
            Item::CopperIngot => 3,
            Item::IronIngot => 4,
            Item::Gear => 5,
            Item::Steel => 6,
            Item::Wire => 7,
            Item::ReinforcedSteel => 8,
            Item::ElectricalCircuit => 9,
            Item::MicroProcessor => 10,
            Item::RotorBlade => 11,
            Item::Propeller => 12,
            Item::BigPropeller => 13,
            Item::Hull => 14,
            Item::Motor => 15,
            Item::Battery => 16,
            Item::ControlModule => 17,
            Item::HelicopterFrame => 18,
            Item::Engine => 19,
            Item::Helicopter => 20,
        })
    }
}
