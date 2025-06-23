use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::ItemType,
        machine_types::{InputItems, MachineType, OutputItems, Side},
    },
    plugins::world::MiddlegroundObject,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Void;

#[typetag::serde]
impl MachineType for Void {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        _output_items: Option<&mut OutputItems>,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        input_items.all().clear();
    }

    fn can_accept(
        &self,
        _item: &ItemType,
        _input_items: &InputItems,
        _output_items: Option<&OutputItems>,
        _input_side: &Side,
    ) -> bool {
        true
    }
}
