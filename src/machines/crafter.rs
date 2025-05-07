use std::collections::VecDeque;

use crate::plugins::world::MiddlegroundObject;

use super::{Item, MachineType};

#[derive(Debug, Clone, Copy)]
pub struct Crafter;

#[allow(unused)] // [TODO] remove this once the crafter is implemented
impl MachineType for Crafter {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        todo!()
    }

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
