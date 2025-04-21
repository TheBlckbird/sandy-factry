use bevy::prelude::*;
use std::fmt::Debug;

pub type Item = i32;

#[derive(Debug)]
pub struct Building {
    pub building_type: Box<dyn BuildingType>,
    pub items: Vec<Item>,
    pub position: (u8, u8),
}

impl Building {
    pub fn new(building_type: Box<dyn BuildingType>, position: (u8, u8)) -> Self {
        Self {
            building_type,
            items: Vec::new(),
            position,
        }
    }

    pub fn perform_action(&mut self) -> Result<Option<Item>, ()> {
        if self.items.len() != self.building_type.get_input_count() {
            return Err(());
        }

        let output = self.building_type.perform_action(&self.items);
        self.items = Vec::new();
        output
    }
}

pub trait BuildingType: Debug + Send + Sync {
    fn perform_action(&self, contained_items: &[Item]) -> Result<Option<Item>, ()>;
    fn get_input_count(&self) -> usize;
}
