use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use std::{collections::VecDeque, fmt::Debug};

use crate::plugins::world::MiddlegroundObject;

pub mod belt;
pub mod crafter;
pub mod miner;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Item {
    Coal,
    RawCopper,
    RawIron,
    CopperIngot,
    IronIngot,
}

impl From<Item> for TileTextureIndex {
    fn from(value: Item) -> Self {
        TileTextureIndex(match value {
            Item::Coal => 0,
            Item::RawCopper => 1,
            Item::RawIron => 2,
            Item::CopperIngot => 3,
            Item::IronIngot => 4,
        })
    }
}

#[derive(Debug, Component)]
pub struct Machine {
    pub machine_type: Box<dyn MachineType>,
    pub input_items: VecDeque<Item>,
    pub output_items: VecDeque<Item>,
}

impl Machine {
    pub fn new(
        machine_type: Box<dyn MachineType>,
        input_items: VecDeque<Item>,
        output_items: VecDeque<Item>,
    ) -> Self {
        Self {
            machine_type,
            input_items,
            output_items,
        }
    }

    pub fn perform_action(&mut self, middleground_object: Option<MiddlegroundObject>) {
        // if self.input_items.len() != self.building_type.get_input_count() {
        //     return;
        // }

        self.machine_type.perform_action(
            &mut self.input_items,
            &mut self.output_items,
            middleground_object,
        );
    }
}

pub trait MachineType: Debug + Send + Sync {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        middleground_object: Option<MiddlegroundObject>,
    );
    // fn get_input_count(&self) -> usize;
    fn clone_box(&self) -> Box<dyn MachineType>;
    fn can_accept(
        &self,
        item: &Item,
        input_items: &VecDeque<Item>,
        output_items: &VecDeque<Item>,
    ) -> bool;
}
