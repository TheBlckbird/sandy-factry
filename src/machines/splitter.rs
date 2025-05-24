use crate::plugins::world::MiddlegroundObject;

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
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        // Get current input side
        let current_output_side_index = match self.last_output_side_index {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };

        let current_output_side = self.output_sides[current_output_side_index];

        //  Check if there are any items in the output and if something can be pulled from the input
        if output_items.is_empty()
            && let Some(input_item) = input_items.exactly_one_mut().pop_front()
        {
            output_items
                .get_side_mut(&current_output_side)
                .as_mut()
                .unwrap_or_else(|| {
                    panic!("Splitter should have the output side `{current_output_side:?}`")
                })
                .push_back(input_item);

            self.last_output_side_index = current_output_side_index;
        }
    }

    fn clone_box(&self) -> Box<dyn MachineType> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        _item: &super::Item,
        input_items: &super::InputItems,
        output_items: &super::OutputItems,
        _input_side: &super::Side,
    ) -> bool {
        input_items.count() + output_items.count() < 1
    }
}
