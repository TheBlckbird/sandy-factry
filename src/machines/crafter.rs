use std::collections::HashMap;

use crate::plugins::{crafting::recipe_types::CrafterRecipe, world::MiddlegroundObject};

use super::{InputItems, Side, Item, MachineType, OutputItems};

#[derive(Debug, Clone, Default)]
pub struct Crafter {
    current_recipe: Option<CrafterRecipe>,
    /// Burn time left
    burn_time: u8,
    /// Crafting time left
    /// `None` if nothing is currently being crafting
    crafting_time_left: Option<u8>,
}

impl Crafter {
    const MAX_BURN_TIME: u8 = 100;
    const COAL_BURN_TIME: u8 = 50;
    const CRAFTING_BURN_TIME: u8 = 10;

    pub fn new(current_recipe: CrafterRecipe) -> Self {
        Self {
            current_recipe: Some(current_recipe),
            burn_time: 0,
            crafting_time_left: None,
        }
    }
}

impl MachineType for Crafter {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        // Convert the coal to burn time

        let coal_input = input_items
            .west
            .as_mut()
            .expect("A Crafter should have a west input");

        // We already know, there's only coal in here
        if coal_input.len() == 1 {
            self.burn_time += Self::COAL_BURN_TIME;
        } else if !coal_input.is_empty() {
            panic!("There should only be one single coal in the miner fuel input");
        }

        coal_input.clear();

        // Crafting

        let current_recipe = match &self.current_recipe {
            Some(current_recipe) => current_recipe,
            None => return,
        };

        let mut unset_crafting_time = false;

        // Check whether something is currently being crafted
        match self.crafting_time_left.as_mut() {
            Some(crafting_time_left) => {
                if *crafting_time_left == 0 {
                    // Crafting finished
                    // Append the crafted item to `output_items`
                    for _ in 0..current_recipe.output_count {
                        output_items.push_back(current_recipe.output_item);
                    }

                    unset_crafting_time = true;
                } else {
                    // Crafting not finished,
                    *crafting_time_left -= 1;
                }
            }
            None => {
                if self.burn_time >= Self::CRAFTING_BURN_TIME {
                    let mut items = HashMap::new();
                    let items_input = input_items
                        .north
                        .as_mut()
                        .expect("A Crafter should have a north input");

                    // Convert the queue into a HashMap of all the items and their count
                    for item in items_input.iter() {
                        items
                            .entry(*item)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }

                    // Try the crafting recipe and return this function if there aren't enough items
                    // else assign the HashMap of the remaining items to `rest_items`
                    let rest_items = match current_recipe.try_crafting(&items) {
                        Some(rest_items) => rest_items,
                        None => return,
                    };

                    items_input.clear();
                    self.burn_time -= Self::CRAFTING_BURN_TIME;

                    // Transfer the `rest_items` back into `items_input`
                    for (item, count) in rest_items.into_iter() {
                        for _ in 0..count {
                            items_input.push_back(item);
                        }
                    }

                    self.crafting_time_left = Some(current_recipe.crafting_time);
                }
            }
        }

        if unset_crafting_time {
            self.crafting_time_left = None;
        }
    }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(self.clone())
    }

    fn can_accept(
        &self,
        item: &Item,
        _input_items: &InputItems,
        _output_items: &OutputItems,
        input_side: &Side,
    ) -> bool {
        match input_side {
            crate::Direction::North => true,
            crate::Direction::West => {
                *item == Item::Coal && Self::MAX_BURN_TIME - self.burn_time >= Self::COAL_BURN_TIME
            }
            _ => unreachable!(),
        }
    }
}
