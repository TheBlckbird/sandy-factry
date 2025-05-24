use crate::plugins::world::MiddlegroundObject;
use sandy_factry_macros::MachineVariants;

use super::{InputItems, Item, MachineType, MachineVariants, OutputItems, Side};

#[derive(Debug, Clone, Copy)]
pub struct Void;
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

#[derive(Debug, Default, MachineVariants)]
#[machine_type(Void)]
#[machine(Void)]
#[render]
pub enum VoidVariants {
    #[default]
    #[variant(inputs(North, East, South, West), texture = 28)]
    All,
}
