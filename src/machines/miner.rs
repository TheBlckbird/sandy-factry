use std::collections::VecDeque;

use crate::plugins::world::MiddlegroundObject;

use super::{Item, MachineType};

#[derive(Debug, Clone, Copy, Default)]
pub struct Miner {
    ticks_passed: u32,
    burn_time: u32,
}

impl Miner {
    pub fn new(ticks_passed: u32) -> Self {
        Self {
            ticks_passed,
            burn_time: 0,
        }
    }
}

impl MachineType for Miner {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        println!("Miner");
        if input_items.is_empty() {
            return;
        }

        input_items.pop_front();

        // if self.ticks_passed == 120 && self.burn_time >= 20 {
        // self.ticks_passed = 0;
        // self.burn_time -= 20;

        if output_items.len() < 50 {
            if let Some(middleground_object) = middleground_object {
                let item = match middleground_object {
                    MiddlegroundObject::Coal => Item::Coal,
                    MiddlegroundObject::Copper => Item::Copper,
                    MiddlegroundObject::Iron => Item::Iron,
                };

                output_items.push_back(item);
            }
        }
        // }

        // self.ticks_passed += 1;
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
        /*self.burn_time <= 600 &&*/
        item == &Item::Coal
    }
}
