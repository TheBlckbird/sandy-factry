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
    #[strum(to_string = "Iron Plate")]
    IronPlate,
    #[strum(to_string = "Copper Plate")]
    CopperPlate,
    #[strum(to_string = "Gear")]
    Gear,
    #[strum(to_string = "Steel")]
    Steel,
}

impl From<Item> for TileTextureIndex {
    fn from(value: Item) -> Self {
        TileTextureIndex(match value {
            Item::Coal => 0,
            Item::RawCopper => 1,
            Item::RawIron => 2,
            Item::CopperIngot => 3,
            Item::IronIngot => 4,
            Item::IronPlate => 4,
            Item::CopperPlate => 4,
            Item::Gear => 4,
            Item::Steel => 4,
        })
    }
}
