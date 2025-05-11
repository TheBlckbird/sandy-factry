use crate::plugins::world::MiddlegroundObject;

use super::{InputItems, InputSide, Item, MachineType, OutputItems};

#[derive(Debug, Clone, Copy, Default)]
pub struct Miner;
impl MachineType for Miner {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        if output_items.len() < 50
            && let Some(middleground_object) = middleground_object
            // We know for sure that only coal can be in the input items so we check for that
            && !input_items.exactly_one().is_empty()
        {
            let item = match middleground_object {
                MiddlegroundObject::Coal => Item::Coal,
                MiddlegroundObject::Copper => Item::RawCopper,
                MiddlegroundObject::Iron => Item::RawIron,
            };

            // Remove one coal, if mining was successful
            input_items.exactly_one_mut().pop_front();
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
        _input_items: &InputItems,
        _output_items: &OutputItems,
        _input_side: &InputSide,
    ) -> bool {
        item == &Item::Coal
    }
}
