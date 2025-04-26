use bevy::prelude::*;

use std::{collections::VecDeque, fmt::Debug};

pub mod belt;
pub mod crafter;
pub mod miner;

pub type Item = i32;

#[derive(Debug, Component)]
pub struct Building {
    pub building_type: Box<dyn BuildingType>,
    pub input_items: VecDeque<Item>,
    pub output_items: VecDeque<Item>,
}

impl Building {
    pub fn new(
        building_type: Box<dyn BuildingType>,
        input_items: VecDeque<Item>,
        output_items: VecDeque<Item>,
    ) -> Self {
        Self {
            building_type,
            input_items,
            output_items,
        }
    }

    pub fn perform_action(&mut self) {
        if self.input_items.len() != self.building_type.get_input_count() {
            return;
        }

        self.building_type
            .perform_action(&mut self.input_items, &mut self.output_items);
    }
}

pub trait BuildingType: Debug + Send + Sync {
    fn perform_action(&self, input_items: &mut VecDeque<Item>, output_items: &mut VecDeque<Item>);
    fn get_input_count(&self) -> usize;
    fn clone_box(&self) -> Box<dyn BuildingType>;
}
