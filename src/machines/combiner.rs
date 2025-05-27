use serde::{Deserialize, Serialize};

use super::{InputItems, Item, MachineType, OutputItems, Side};

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
        output_items: &mut OutputItems,
        _middleground_object: Option<crate::plugins::world::MiddlegroundObject>,
    ) {
        // Get current input side
        let current_input_side_index = match self.last_input_side_index {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };

        let current_input_side = self.input_sides[current_input_side_index];

        // Check if there are any items on the combiner
        if output_items.exactly_one().is_empty()
            && let Some(input_item) = input_items
                .get_side_mut(&current_input_side)
                .as_mut()
                .unwrap_or_else(|| {
                    panic!("Combiner should have an input at {current_input_side:?}")
                })
                .pop_front()
        {
            output_items.exactly_one_mut().push_back(input_item);
        }

        self.last_input_side_index = current_input_side_index;
    }

    fn clone_box(&self) -> Box<dyn MachineType> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        _item: &Item,
        input_items: &InputItems,
        output_items: &OutputItems,
        input_side: &Side,
    ) -> bool {
        // Only one item is allowed in the combiner
        if input_items.count() + output_items.count() > 0 {
            return false;
        }

        let input_side_index = match self.last_input_side_index {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };

        input_side == &self.input_sides[input_side_index]
    }
}
