use bevy::prelude::*;
use recipe_types::{CrafterRecipe, FurnaceRecipe};

use crate::machines::Item;

mod recipe_types;

#[derive(Debug, Resource)]
pub struct CrafterRecipes(Vec<CrafterRecipe>);

#[derive(Debug, Resource)]
pub struct FurnaceRecipes(Vec<FurnaceRecipe>);

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CrafterRecipes(vec![
            // [TODO] Remove test recipe
            CrafterRecipe::new(Box::new([Item::RawCopper, Item::RawIron]), Item::Coal, 1, 1),
        ]))
        .insert_resource(FurnaceRecipes(vec![]))
        .add_systems(Startup, list_all_recipes);
    }
}

// [DEBUG] Remove this
fn list_all_recipes(crafter_recipes: Res<CrafterRecipes>) {
    for recipe in crafter_recipes.0.iter() {
        println!(
            "The Recipe for {} {:?} uses the ingredients {:?} and takes {} second{}",
            recipe.output_count,
            recipe.output_item,
            recipe.ingredients,
            recipe.crafting_time,
            if recipe.crafting_time == 1 { "" } else { "s" }
        )
    }
}
