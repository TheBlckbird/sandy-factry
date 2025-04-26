use std::collections::VecDeque;

use super::{BuildingType, Item};

#[derive(Debug, Clone, Copy)]
pub struct Crafter;
impl BuildingType for Crafter {
    fn perform_action(&self, input_items: &mut VecDeque<Item>, output_items: &mut VecDeque<Item>) {
        todo!()
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}
