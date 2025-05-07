use std::collections::VecDeque;

use crate::plugins::world::MiddlegroundObject;

use super::{Item, MachineType};

#[derive(Debug, Clone, Copy)]
pub struct Belt;
impl MachineType for Belt {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        if output_items.is_empty()
            && let Some(input_item) = input_items.pop_front()
        {
            output_items.push_back(input_item);
        }
    }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        _item: &Item,
        input_items: &VecDeque<Item>,
        output_items: &VecDeque<Item>,
    ) -> bool {
        (input_items.len() + output_items.len()) == 0
    }
}
