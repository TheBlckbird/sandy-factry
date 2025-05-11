use std::collections::HashMap;

use crate::plugins::{crafting::recipe_types::CrafterRecipe, world::MiddlegroundObject};

use super::{InputItems, InputSide, Item, MachineType, OutputItems};

#[derive(Debug, Clone, Default)]
pub struct Crafter {
    current_recipe: Option<CrafterRecipe>,
}

impl Crafter {
    pub fn new(current_recipe: CrafterRecipe) -> Self {
        Self {
            current_recipe: Some(current_recipe),
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
        let current_recipe = match &self.current_recipe {
            Some(current_recipe) => current_recipe,
            None => return,
        };

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

        // Transfer the `rest_items` back into `items_input`
        for (item, count) in rest_items.into_iter() {
            for _ in 0..count {
                items_input.push_back(item);
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
        _input_items: &InputItems,
        _output_items: &OutputItems,
        _input_side: &InputSide,
    ) -> bool {
        true
    }
}
