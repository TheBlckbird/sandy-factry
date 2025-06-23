use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    content::{
        items::ItemType,
        machine_types::{
            InputItems, MachineType, OutputItems, Side, UnwrapOutputItems, UnwrapOutputItemsMut,
        },
    },
    plugins::{crafting::recipe_types::CrafterRecipe, world::MiddlegroundObject},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crafter {
    pub current_recipe: Option<CrafterRecipe>,

    /// Crafting time left
    /// `None` if nothing is currently being crafting
    crafting_time_left: Option<(u16, (ItemType, u8))>,
}

impl Crafter {
    pub fn new() -> Self {
        Self {
            current_recipe: None,
            crafting_time_left: None,
        }
    }
}

#[typetag::serde]
impl MachineType for Crafter {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        mut output_items: Option<&mut OutputItems>,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        // Crafting

        let current_recipe = match &self.current_recipe {
            Some(current_recipe) => current_recipe,
            None => return,
        };

        // let mut unset_crafting_time = false;

        // Check whether something is currently being crafted
        match self.crafting_time_left.as_mut() {
            Some((0, (output_item, output_count))) => {
                // Crafting finished
                // Append the crafted item to `output_items`

                for _ in 0..*output_count {
                    output_items
                        .unwrap_single_side_mut()
                        .push_back((*output_item).into());
                }

                self.crafting_time_left = None;
            }

            Some((crafting_time_left, _)) => {
                *crafting_time_left -= 1;
            }

            None => {
                // Only try to craft something, if there are no items already crafted
                if !output_items.unwrap_single_side().is_empty() {
                    return;
                }

                let mut items = HashMap::new();
                let items_input = input_items.exactly_one_mut();

                // Convert the queue into a HashMap of all the items and their count
                for item in items_input.iter() {
                    items
                        .entry(**item)
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

                // Transfer the `rest_items` back into `items_input`
                for (item, count) in rest_items.into_iter() {
                    for _ in 0..count {
                        items_input.push_back(item.into());
                    }
                }

                self.crafting_time_left = Some((
                    current_recipe.crafting_time,
                    (current_recipe.output_item, current_recipe.output_count),
                ));
            }
        }
    }

    fn can_accept(
        &self,
        item: &ItemType,
        input_items: &InputItems,
        _output_items: Option<&OutputItems>,
        _input_side: &Side,
    ) -> bool {
        input_items.count_item(item) < 50
    }

    fn is_selectable(&self) -> bool {
        true
    }
}
