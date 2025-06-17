use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::ItemType,
        machine_types::{InputItems, MachineType, OutputItems, Side},
    },
    plugins::world::MiddlegroundObject,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Belt;

#[typetag::serde]
impl MachineType for Belt {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        if output_items.exactly_one().is_empty()
            && let Some(input_item) = input_items.exactly_one_mut().pop_front()
        {
            output_items.exactly_one_mut().push_back(input_item);
        }
    }

    fn can_accept(
        &self,
        _item: &ItemType,
        input_items: &InputItems,
        output_items: &OutputItems,
        _input_side: &Side,
    ) -> bool {
        (input_items.exactly_one().len() + output_items.exactly_one().len()) == 0
    }

    fn tick_after_first(&self) -> bool {
        true
    }
}
