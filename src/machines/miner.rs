use serde::{Deserialize, Serialize};

use crate::plugins::world::MiddlegroundObject;

use super::{InputItems, Item, MachineType, OutputItems, Side};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Miner {
    /// burn time left
    burn_time: u8,

    /// time spent mining the current resource
    mining_time: Option<u8>,
}

impl Miner {
    const MAX_BURN_TIME: u8 = 100;
    const SINGLE_COAL_BURN_TIME: u8 = 30;
    const MINING_BURN_TIME: u8 = 10;
    const MINING_TIME: u8 = 30;

    pub fn new() -> Self {
        Self {
            burn_time: 0,
            mining_time: None,
        }
    }
}

#[typetag::serde]
impl MachineType for Miner {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        middleground_object: Option<MiddlegroundObject>,
    ) {
        // Convert the coal to burn time
        // We already know, there's only one coal in here
        if self.burn_time + Self::SINGLE_COAL_BURN_TIME <= Self::MAX_BURN_TIME
            && input_items.exactly_one_mut().pop_front().is_some()
        {
            self.burn_time += Self::SINGLE_COAL_BURN_TIME;
        }

        input_items.exactly_one_mut().clear();

        // Mining

        if output_items.exactly_one().len() < 50
            && self.burn_time >= Self::MINING_BURN_TIME
            && let Some(middleground_object) = middleground_object
        {
            match &mut self.mining_time {
                Some(0) | None => {
                    let item = match middleground_object {
                        MiddlegroundObject::Coal => Item::Coal,
                        MiddlegroundObject::Copper => Item::RawCopper,
                        MiddlegroundObject::Iron => Item::RawIron,
                    };

                    // Remove one coal, if mining was successful
                    input_items.exactly_one_mut().pop_front();
                    // Append the resource under the miner
                    output_items.exactly_one_mut().push_back(item);

                    self.burn_time -= Self::MINING_BURN_TIME;
                    self.mining_time = Some(Self::MINING_TIME);
                }
                Some(mining_time) => {
                    *mining_time -= 1;
                }
            }
        }
    }

    fn can_accept(
        &self,
        item: &Item,
        input_items: &InputItems,
        _output_items: &OutputItems,
        _input_side: &Side,
    ) -> bool {
        *item == Item::Coal && input_items.count() < 50
    }
}
