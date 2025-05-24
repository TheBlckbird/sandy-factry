use crate::plugins::world::MiddlegroundObject;

use super::{InputItems, Item, MachineType, OutputItems, Side};

#[derive(Debug, Clone, Copy)]
pub struct Belt;
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

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        _item: &Item,
        input_items: &InputItems,
        output_items: &OutputItems,
        _input_side: &Side,
    ) -> bool {
        (input_items.exactly_one().len() + output_items.exactly_one().len()) == 0
    }
}
