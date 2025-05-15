use sandy_factry_macros::MachineVariants;

use super::{InputItems, Item, MachineType, MachineVariants, OutputItems, Side};

#[derive(Debug, Clone, Copy)]
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
        if output_items.is_empty()
            && let Some(input_item) = input_items
                .get_side_mut(&current_input_side)
                .as_mut()
                .unwrap_or_else(|| {
                    panic!("Combiner should have an input at {current_input_side:?}")
                })
                .pop_front()
        {
            output_items.push_back(input_item);
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
        if input_items.count() + output_items.len() > 0 {
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

#[derive(Debug, Default, MachineVariants)]
#[machine_type(Combiner)]
#[render]
pub enum CombinerVariants {
    #[default]
    #[variant(inputs(North, West), outputs(South), texture = 14, machine = Combiner::new([Side::North, Side::West]))]
    UpLeft,
    #[variant(inputs(West, South), outputs(East), texture = 15, machine = Combiner::new([Side::West, Side::South]))]
    LeftDown,
    #[variant(inputs(South, East), outputs(North), texture = 16, machine = Combiner::new([Side::South, Side::East]))]
    DownRight,
    #[variant(inputs(East, North), outputs(West), texture = 17, machine = Combiner::new([Side::East, Side::North]))]
    RightUp,
    #[variant(inputs(South, West), outputs(North), texture = 18, machine = Combiner::new([Side::South, Side::West]))]
    DownLeft,
    #[variant(inputs(West, North), outputs(East), texture = 19, machine = Combiner::new([Side::West, Side::North]))]
    LeftUp,
    #[variant(inputs(North, East), outputs(South), texture = 20, machine = Combiner::new([Side::North, Side::East]))]
    UpRight,
    #[variant(inputs(East, South), outputs(West), texture = 21, machine = Combiner::new([Side::East, Side::South]))]
    RightDown,
}
