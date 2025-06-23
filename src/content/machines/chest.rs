use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::ItemType,
        machine_types::{InputItems, MachineType, OutputItems, Side, UnwrapOutputItemsMut},
    },
    plugins::world::MiddlegroundObject,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Chest {
    pub last_side: Side,
}

impl Chest {
    pub fn new() -> Self {
        Self {
            last_side: Side::North,
        }
    }
}

#[typetag::serde]
impl MachineType for Chest {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        mut output_items: Option<&mut OutputItems>,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        let sides = match self.last_side {
            Side::North => [Side::North, Side::East, Side::South, Side::West],
            Side::East => [Side::East, Side::South, Side::West, Side::North],
            Side::South => [Side::South, Side::West, Side::North, Side::East],
            Side::West => [Side::West, Side::North, Side::East, Side::South],
        };

        let output_items = output_items.unwrap_multiple_sides_mut();
        output_items.preferred_sides.clear();

        for side in sides {
            if let Some(item) = input_items
                .get_side_mut(&side)
                .expect("Chest should have all inputs")
                .pop_front()
            {
                output_items.items.push_back(item);
            }

            output_items.preferred_sides.push(side);
        }

        if !output_items.items.is_empty() {
            self.last_side = match self.last_side {
                Side::North => Side::East,
                Side::East => Side::South,
                Side::South => Side::West,
                Side::West => Side::North,
            };
        }
    }

    fn can_accept(
        &self,
        _item: &ItemType,
        _input_items: &InputItems,
        _output_items: Option<&OutputItems>,
        _input_side: &Side,
    ) -> bool {
        true
    }
}
