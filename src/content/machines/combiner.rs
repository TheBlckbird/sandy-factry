use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::ItemType,
        machine_types::{
            InputItems, MachineType, OutputItems, Side, UnwrapOutputItems, UnwrapOutputItemsMut,
        },
    },
    plugins::world::MiddlegroundObject,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Combiner {
    input_sides: [Side; 2],
    last_input_side_index: usize,
}

impl Combiner {
    pub fn new(input_sides: [Side; 2]) -> Self {
        Self {
            input_sides,
            last_input_side_index: 0,
        }
    }
}

#[typetag::serde]
impl MachineType for Combiner {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        mut output_items: Option<&mut OutputItems>,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        // Get current input side
        let current_input_side_index = match self.last_input_side_index {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };

        let current_input_side = self.input_sides[current_input_side_index];

        // Check if there are any items on the combiner
        if output_items.unwrap_single_side().is_empty()
            && let Some(input_item) = input_items
                .get_side_mut(&current_input_side)
                .as_mut()
                .unwrap_or_else(|| {
                    panic!("Combiner should have an input at {current_input_side:?}")
                })
                .pop_front()
        {
            output_items.unwrap_single_side_mut().push_back(input_item);
        }

        self.last_input_side_index = current_input_side_index;
    }

    fn can_accept(
        &self,
        _item: &ItemType,
        input_items: &InputItems,
        output_items: Option<&OutputItems>,
        input_side: &Side,
    ) -> bool {
        // Only one item is allowed in the combiner
        if input_items.count()
            + output_items
                .expect("Combiner should have output items")
                .len()
            > 0
        {
            return false;
        }

        let input_side_index = match self.last_input_side_index {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };

        input_side == &self.input_sides[input_side_index]
    }

    fn tick_after_first(&self) -> bool {
        true
    }
}
