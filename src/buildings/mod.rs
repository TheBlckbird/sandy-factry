use std::fmt::Debug;

// use crate::{NumberItem, buildings::end::End};

type Item = i32;

#[derive(Debug)]
pub struct TestBuilding {
    pub building_type: Box<dyn BuildingType>,
    pub numbers: Vec<Item>,
    pub position: (u8, u8),
}

impl TestBuilding {
    pub fn new(building_type: Box<dyn BuildingType>, position: (u8, u8)) -> Self {
        Self {
            building_type,
            numbers: Vec::new(),
            position,
        }
    }

    pub fn perform_action(&mut self) -> Result<Option<Item>, ()> {
        if self.numbers.len() != self.building_type.get_input_count() {
            return Err(());
        }

        let output = self.building_type.perform_action(&self.numbers);
        self.numbers = Vec::new();
        output
    }
}

// impl Default for Building {
//     fn default() -> Self {
//         Self {
//             building_type: Box::new(End),
//             numbers: Default::default(),
//             position: Default::default(),
//         }
//     }
// }

pub trait BuildingType: Debug + Send + Sync {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()>;
    fn get_input_count(&self) -> usize;
    fn render(&self) -> char;
}
