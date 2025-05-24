use crate::plugins::world::MiddlegroundObject;
use sandy_factry_macros::MachineVariants;

use super::{InputItems, Item, MachineType, MachineVariants, OutputItems, Side};

#[derive(Debug, Clone, Copy)]
pub struct Belt;
impl MachineType for Belt {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        if output_items.exactly_one().is_empty()
            && let Some(input_item) = input_items.exactly_one_mut().pop_front()
        {
            output_items.exactly_one_mut().push_back(input_item);
        }
    }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(*self)
    }

    fn can_accept(
        &self,
        _item: &Item,
        input_items: &InputItems,
        output_items: &OutputItems,
        _input_side: &Side,
    ) -> bool {
        (input_items.exactly_one().len() + output_items.exactly_one().len()) == 0
    }
}

#[derive(Debug, Default, MachineVariants)]
#[machine_type(Belt)]
#[machine(Belt)]
#[render]
pub enum BeltVariants {
    #[default]
    #[variant(inputs(South), outputs(North), texture = 0)]
    Up,
    #[variant(inputs(North), outputs(South), texture = 1)]
    Down,
    #[variant(inputs(East), outputs(West), texture = 2)]
    Left,
    #[variant(inputs(West), outputs(East), texture = 3)]
    Right,
    #[variant(inputs(South), outputs(East), texture = 4)]
    DownRight,
    #[variant(inputs(West), outputs(South), texture = 5)]
    LeftDown,
    #[variant(inputs(North), outputs(West), texture = 6)]
    UpLeft,
    #[variant(inputs(East), outputs(North), texture = 7)]
    RightUp,
    #[variant(inputs(East), outputs(South), texture = 8)]
    RightDown,
    #[variant(inputs(South), outputs(West), texture = 9)]
    DownLeft,
    #[variant(inputs(West), outputs(North), texture = 10)]
    LeftUp,
    #[variant(inputs(North), outputs(East), texture = 11)]
    UpRight,
}
