use std::collections::VecDeque;

use crate::plugins::world::MiddlegroundObject;

use super::{MachineType, Item};

#[derive(Debug, Clone, Copy)]
pub struct Crafter;
impl MachineType for Crafter {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        todo!()
    }

    // fn get_input_count(&self) -> usize {
    //     1
    // }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        item: &Item,
        input_items: &VecDeque<Item>,
        output_items: &VecDeque<Item>,
    ) -> bool {
        todo!()
    }
}
