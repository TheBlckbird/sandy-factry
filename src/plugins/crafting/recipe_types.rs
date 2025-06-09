use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::content::items::Item;

pub enum Recipe {
    Crafter(CrafterRecipe),
    Furnace(FurnaceRecipe),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrafterRecipe {
    pub ingredients: HashMap<Item, u16>,
    pub output_item: Item,
    pub output_count: u8,
    pub crafting_time: u16,
}

impl CrafterRecipe {
    #[allow(unused)]
    pub fn new(
        ingredients: HashMap<Item, u16>,
        output_item: Item,
        output_count: u8,
        crafting_time: u16,
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
        let mut remaining_ingredients = external_ingredients.clone();

        // Check for each ingredient if it exists in the provided ingredients
        for (item, &required_item_count) in &self.ingredients {
            let external_item_count = remaining_ingredients.get_mut(item)?;

            if *external_item_count < required_item_count {
                return None;
            }

            *external_item_count -= required_item_count;

            // Remove the entry completely from the HashMap if the count is zero after this
            // (This is theoretically irrelevant with the current crafter implementation, but ´
            // I'm still doing this, in case I ever change this code)
            if *external_item_count == 0 {
                remaining_ingredients.remove(item);
            }
        }

        Some(remaining_ingredients)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FurnaceRecipe {
    pub ingredient: (Item, u16),
    pub output_item: (Item, u16),
    pub burn_time: u8,
}

impl FurnaceRecipe {
    #[allow(unused)]
    pub fn new(output_item: (Item, u16), ingredient: (Item, u16), burn_time: u8) -> Self {
        Self {
            ingredient,
            output_item,
            burn_time,
        }
    }

    pub fn try_crafting(
        &self,
        external_ingredients: &HashMap<Item, u16>,
    ) -> Option<HashMap<Item, u16>> {
        let mut remaining_ingredients = external_ingredients.clone();

        // Check for each ingredient if it exists in the provided ingredients
        if let Some(external_item_count) = remaining_ingredients.get_mut(&self.ingredient.0)
            && *external_item_count >= self.ingredient.1
        {
            *external_item_count -= self.ingredient.1;

            // Remove the entry completely from the HashMap if the count is zero after this
            // (This is theoretically irrelevant with the current crafter implementation, but ´
            // I'm still doing this, in case I ever change this code)
            if *external_item_count == 0 {
                remaining_ingredients.remove(&self.ingredient.0);
            }
        } else {
            return None;
        }

        Some(remaining_ingredients)
    }
}
