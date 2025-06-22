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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Miner {
    /// time spent mining the current resource
    mining_time: Option<u8>,
}

impl Miner {
    const MINING_TIME: u8 = 30;

    pub fn new() -> Self {
        Self { mining_time: None }
    }
}

#[typetag::serde]
impl MachineType for Miner {
    fn perform_action(
        &mut self,
        _input_items: &mut InputItems,
        mut output_items: Option<&mut OutputItems>,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        // Mining

        if output_items.unwrap_single_side().len() < 50
            && let Some(middleground_object) = middleground_object
        {
            match &mut self.mining_time {
                Some(0) => {
                    let item = match middleground_object {
                        MiddlegroundObject::Coal => ItemType::Coal,
                        MiddlegroundObject::Copper => ItemType::RawCopper,
                        MiddlegroundObject::Iron => ItemType::RawIron,
                    };

                    // Append the resource under the miner
                    output_items.unwrap_single_side_mut().push_back(item.into());

                    self.mining_time = None;
                }
                // Reduce the mining timer by one if it is set
                Some(mining_time) => {
                    *mining_time -= 1;
                }
                // If there are no other items in the output, reset the timer
                None if output_items.unwrap_single_side().is_empty() => {
                    self.mining_time = Some(Self::MINING_TIME);
                }
                // else do nothing and try again next time
                None => {}
            }
        }
    }

    fn can_accept(
        &self,
        _item: &ItemType,
        _input_items: &InputItems,
        _output_items: Option<&OutputItems>,
        _input_side: &Side,
    ) -> bool {
        unreachable!()
    }
}
