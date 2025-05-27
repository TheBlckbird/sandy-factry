use serde::{Deserialize, Serialize};

use crate::plugins::world::MiddlegroundObject;

use super::{InputItems, Item, MachineType, OutputItems, Side};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Void;

#[typetag::serde]
impl MachineType for Void {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        _output_items: &mut OutputItems,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        input_items.all().clear();
    }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        _item: &Item,
        _input_items: &InputItems,
        _output_items: &OutputItems,
        _input_side: &Side,
    ) -> bool {
        true
    }
}
