use std::collections::VecDeque;

use super::{BuildingType, Item};

#[derive(Debug, Clone, Copy)]
pub struct Belt;
impl BuildingType for Belt {
    fn perform_action(&self, input_items: &mut VecDeque<Item>, output_items: &mut VecDeque<Item>) {
        if output_items.is_empty() {
            output_items.push_back(input_items.pop_front().unwrap());
        }
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}
