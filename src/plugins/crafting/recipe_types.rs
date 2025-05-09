use crate::machines::Item;

#[derive(Debug)]
pub struct CrafterRecipe {
    pub ingredients: Box<[Item]>,
    pub output_item: Item,
    pub output_count: u8,
    pub crafting_time: u8,
}

impl CrafterRecipe {
    pub fn new(
        ingredients: Box<[Item]>,
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
