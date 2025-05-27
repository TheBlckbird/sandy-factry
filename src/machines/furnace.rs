use std::collections::HashMap;

use crate::plugins::{crafting::recipe_types::FurnaceRecipe, world::MiddlegroundObject};

use super::{InputItems, Item, MachineType, OutputItems, Side};

#[derive(Debug, Clone)]
pub struct Furnace {
    current_recipe: Option<FurnaceRecipe>,

    /// burn time left
    burn_time: u8,

    /// Crafting time left
    /// `None` if nothing is currently being crafting
    crafting_time_left: Option<u8>,

    /// The side where items are inputted
    input_side: Side,

    /// The side where coal is inputted
    coal_input_side: Side,
}

impl Furnace {
    const MAX_BURN_TIME: u8 = 100;
    const COAL_BURN_TIME: u8 = 60;
    const SMELTING_BURN_TIME: u8 = 15;

    pub fn new(input_side: Side, coal_input_side: Side) -> Self {
        Self {
            current_recipe: None,
            burn_time: 0,
            crafting_time_left: None,
            input_side,
            coal_input_side,
        }
    }

    pub fn new_with_recipe(recipe: FurnaceRecipe, input_side: Side, coal_input_side: Side) -> Self {
        Self {
            current_recipe: Some(recipe),
            burn_time: 0,
            crafting_time_left: None,
            input_side,
            coal_input_side,
        }
    }
}

impl MachineType for Furnace {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        // Convert the coal to burn time

        let coal_input = input_items
            .get_side_mut(&self.coal_input_side)
            .unwrap_or_else(|| {
                panic!(
                    "A Furnace should have the coal input at {:?}",
                    self.coal_input_side
                )
            });

        // We already know, there's only coal in here
        if self.burn_time + Self::COAL_BURN_TIME <= Self::MAX_BURN_TIME
            && coal_input.pop_front().is_some()
        {
            self.burn_time += Self::COAL_BURN_TIME;
        } else if !coal_input.is_empty() {
            panic!("There should only be one single coal in the miner fuel input");
        }

        // Smelting

        let current_recipe = match &self.current_recipe {
            Some(current_recipe) => current_recipe,
            None => return,
        };

        let mut unset_crafting_time = false;

        // Check whether something is currently being smelting
        match self.crafting_time_left.as_mut() {
            Some(crafting_time_left) => {
                if *crafting_time_left == 0 {
                    // Smelting finished
                    // Append the smelted item to `output_items`
                    for _ in 0..current_recipe.output_item.1 {
                        output_items
                            .exactly_one_mut()
                            .push_back(current_recipe.output_item.0);
                    }

                    unset_crafting_time = true;
                } else {
                    // Smelting not finished,
                    *crafting_time_left -= 1;
                }
            }
            None => {
                if self.burn_time >= Self::SMELTING_BURN_TIME {
                    let mut items = HashMap::new();
                    let items_input = input_items
                        .get_side_mut(&self.input_side)
                        .expect("A Furnace should have a north input");

                    // Convert the queue into a HashMap of all the items and their count
                    for item in items_input.iter() {
                        items
                            .entry(*item)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }

                    // Try the furnace recipe and return this function if there aren't enough items
                    // else assign the HashMap of the remaining items to `rest_items`
                    let rest_items = match current_recipe.try_crafting(&items) {
                        Some(rest_items) => rest_items,
                        None => return,
                    };

                    items_input.clear();
                    self.burn_time -= Self::SMELTING_BURN_TIME;

                    // Transfer the `rest_items` back into `items_input`
                    for (item, count) in rest_items.into_iter() {
                        for _ in 0..count {
                            items_input.push_back(item);
                        }
                    }

                    self.crafting_time_left = Some(current_recipe.burn_time);
                }
            }
        }

        if unset_crafting_time {
            self.crafting_time_left = None;
        }
    }

    fn clone_box(&self) -> Box<dyn MachineType> {
        Box::new(self.clone())
    }

    fn can_accept(
        &self,
        item: &Item,
        input_items: &InputItems,
        _output_items: &OutputItems,
        input_side: &Side,
    ) -> bool {
        if *input_side == self.input_side {
            true
        } else if *input_side == self.coal_input_side {
            *item == Item::Coal && input_items.count() < 50
        } else {
            unreachable!()
        }
    }
}
