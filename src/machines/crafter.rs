use std::collections::{HashMap, VecDeque};

use crate::plugins::{crafting::recipe_types::CrafterRecipe, world::MiddlegroundObject};

use super::{Item, MachineType};

#[derive(Debug, Clone, Default)]
pub struct Crafter {
    current_recipe: Option<CrafterRecipe>,
}

impl MachineType for Crafter {
    fn perform_action(
        &mut self,
        input_items: &mut VecDeque<Item>,
        output_items: &mut VecDeque<Item>,
        _middleground_object: Option<MiddlegroundObject>,
    ) {
        // Check if there's currently a recipe set
        let current_recipe = match &self.current_recipe {
            Some(current_recipe) => current_recipe,
            None => return,
        };

        let mut items = HashMap::new();

        // Convert the queue into a HashMap of all the items and their count
        for item in input_items.iter() {
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

        // Transfer the `rest_items` back into `input_items`
        input_items.clear();

        for (item, count) in rest_items.into_iter() {
            for _ in 0..count {
                input_items.push_back(item);
            }
        }

        // Append the crafted item to `output_items`
        for _ in 0..current_recipe.output_count {
            output_items.push_back(current_recipe.output_item);
        }
    }

    fn clone_box(&self) -> Box<(dyn MachineType + 'static)> {
        Box::new(self.clone())
    }

    fn can_accept(
        &self,
        _item: &Item,
        _input_items: &VecDeque<Item>,
        _output_items: &VecDeque<Item>,
    ) -> bool {
        true
    }
}
