use std::collections::HashMap;

use crate::machines::Item;

#[derive(Debug, Clone)]
pub struct CrafterRecipe {
    pub ingredients: HashMap<Item, u16>,
    pub output_item: Item,
    pub output_count: u8,
    pub crafting_time: u8,
}

impl CrafterRecipe {
    pub fn new(
        ingredients: HashMap<Item, u16>,
        output_item: Item,
        output_count: u8,
        crafting_time: u8,
    ) -> Self {
        Self {
            ingredients,
            output_item,
            output_count,
            crafting_time,
        }
    }

    /// Takes another `HashMap` as input and removes the needed ingredients
    /// Returns `None` if there weren't enough ingredients in the provided input and `Some<HashMap<Item, u16>>` otherwise
    pub fn try_crafting(
        &self,
        external_ingredients: &HashMap<Item, u16>,
    ) -> Option<HashMap<Item, u16>> {
        let mut external_ingredients = external_ingredients.clone();

        // Check for each ingredient if it exists in the provided ingredients
        for (item, &required_item_count) in &self.ingredients {
            // Get the ingredient or return `None` if it doesn't exist
            let external_item_count = external_ingredients.get_mut(item)?;

            // Check if there are enough of that item or return `None` if not
            if *external_item_count < required_item_count {
                return None;
            }

            // Remove the specified amount of the item
            *external_item_count -= required_item_count;

            // Remove the entry completely from the HashMap if the count is zero after this
            // (This is theoretically irrelevant with the current crafter implementation, but ´
            // I'm still doing this, in case I ever change this code)
            if *external_item_count == 0 {
                external_ingredients.remove(item);
            }
        }

        // Return the remaining ingredients
        Some(external_ingredients)
    }
}

#[derive(Debug)]
pub struct FurnaceRecipe {
    pub ingredient: Item,
    pub output_item: Item,
    pub burn_time: u8,
}

impl FurnaceRecipe {
    pub fn new(ingredient: Item, output_item: Item, burn_time: u8) -> Self {
        Self {
            ingredient,
            output_item,
            burn_time,
        }
    }
}
