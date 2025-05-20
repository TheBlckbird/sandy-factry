use std::collections::HashMap;

use bevy::prelude::*;
use recipe_types::{CrafterRecipe, FurnaceRecipe};

use crate::machines::Item;

use super::menu::GameState;

pub mod recipe_types;

#[derive(Debug, Resource)]
pub struct CrafterRecipes(Vec<CrafterRecipe>);

#[derive(Debug, Resource)]
pub struct FurnaceRecipes(Vec<FurnaceRecipe>);

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (startup, list_all_recipes).chain(),
        )
        .add_systems(OnExit(GameState::Game), cleanup);
    }
}

fn startup(mut commands: Commands) {
    commands.insert_resource(CrafterRecipes(vec![
        // [TODO] Remove test recipe
        CrafterRecipe::new(
            HashMap::from([(Item::Coal, 1), (Item::RawCopper, 2)]),
            Item::CopperIngot,
            1,
            1,
        ),
    ]));

    commands.insert_resource(FurnaceRecipes(vec![]));
}

// [DEBUG] Remove this
fn list_all_recipes(crafter_recipes: Res<CrafterRecipes>) {
    for recipe in crafter_recipes.0.iter() {
        info!(
            "The Recipe for {} {:?} uses the ingredients {:?} and takes {} second{}",
            recipe.output_count,
            recipe.output_item,
            recipe.ingredients,
            recipe.crafting_time,
            if recipe.crafting_time == 1 { "" } else { "s" }
        )
    }
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
