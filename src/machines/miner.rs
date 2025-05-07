use std::collections::VecDeque;

use crate::plugins::world::MiddlegroundObject;

use super::{Item, MachineType};

#[derive(Debug, Clone, Copy, Default)]
pub struct Miner;
impl MachineType for Miner {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        if output_items.len() < 50
            && let Some(middleground_object) = middleground_object
            // We know for sure that only coal can be in the input items so we check for that
            && !input_items.is_empty()
        {
            let item = match middleground_object {
                MiddlegroundObject::Coal => Item::Coal,
                MiddlegroundObject::Copper => Item::Copper,
                MiddlegroundObject::Iron => Item::Iron,
            };

            // Remove one coal, if mining was successful
            input_items.pop_front();
            // Append the resource under the miner
            output_items.push_back(item);
        }
    }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        item: &Item,
        _input_items: &VecDeque<Item>,
        _output_items: &VecDeque<Item>,
    ) -> bool {
        item == &Item::Coal
    }
}
