use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::ItemType,
        machine_types::{InputItems, MachineType, OutputItems, Side, UnwrapOutputItemsMut},
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
        mut output_items: Option<&mut OutputItems>,
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
        if output_items
            .as_ref()
            .expect("Splitter should have output sides")
            .is_empty()
            && let Some(input_item) = input_items.exactly_one_mut().pop_front()
        {
            // Get output_items (it should always be multiples sides)
            let output_items = output_items.unwrap_multiple_sides_mut();
            // Push item
            output_items.items.push_back(input_item);

            // Set the preferred sides
            output_items.preferred_sides.clear();
            output_items.push_side(current_output_side);
            output_items.push_side(self.output_sides[self.last_output_side_index]);

            // Switch last output side
            self.last_output_side_index = current_output_side_index;
        }
    }

    fn can_accept(
        &self,
        _item: &ItemType,
        input_items: &InputItems,
        output_items: Option<&OutputItems>,
        _input_side: &Side,
    ) -> bool {
        input_items.count()
            + output_items
                .expect("Splitter should have output items")
                .len()
            < 1
    }

    fn tick_after_first(&self) -> bool {
        true
    }
}
