use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::Item,
        machine_types::{InputItems, MachineType, OutputItems, Side},
    },
    plugins::world::MiddlegroundObject,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[typetag::serde]
impl MachineType for Splitter {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
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

    fn can_accept(
        &self,
        _item: &Item,
        input_items: &InputItems,
        output_items: &OutputItems,
        _input_side: &Side,
    ) -> bool {
        input_items.count() + output_items.count() < 1
    }
}
