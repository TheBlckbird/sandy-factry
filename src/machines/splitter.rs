#![allow(unused)] // [TODO] remove when finished

use super::{MachineType, Side};

#[derive(Debug, Clone, Copy)]
pub struct Splitter {
    output_sides: [Side; 2],
    last_output_side_index: usize,
}

impl Splitter {
    pub fn new(output_sides: [Side; 2]) -> Self {
        Self {
            output_sides,
            last_output_side_index: 0,
        }
    }
}

impl MachineType for Splitter {
    fn perform_action(
        &mut self,
        input_items: &mut super::InputItems,
        output_items: &mut super::OutputItems,
        middleground_object: Option<crate::plugins::world::MiddlegroundObject>,
    ) {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn MachineType> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        item: &super::Item,
        input_items: &super::InputItems,
        output_items: &super::OutputItems,
        input_side: &super::Side,
    ) -> bool {
        input_items.count() + output_items.count() < 1
    }
}
