use std::collections::HashMap;

use bevy::prelude::*;
use recipe_types::{CrafterRecipe, FurnaceRecipe};

use crate::machines::Item;

use super::menu::GameState;

pub mod recipe_types;

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct CrafterRecipes(Vec<CrafterRecipe>);

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct FurnaceRecipes(Vec<FurnaceRecipe>);

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), startup)
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

macro_rules! furnace_recipe {
    (out: $output:expr; in: $input:expr; time: $time:expr) => {
        recipe_types::FurnaceRecipe::new(($output, 1), ($input, 1), $time)
    };

    (out: $output:expr, $output_count:expr; in: $input:expr; time: $time:expr) => {
        recipe_types::FurnaceRecipe::new(($output, $output_count), ($input, 1), $time)
    };

    (out: $output:expr; in: $input:expr, $input_count:expr; time: $time:expr) => {
        recipe_types::FurnaceRecipe::new(($output, 1), ($input, $input_count), $time)
    };

    (out: $output:expr, $output_count:expr; in: $input:expr, $input_count:expr; time: $time:expr) => {
        recipe_types::FurnaceRecipe::new(($output, $output_count), ($input, $input_count), $time)
    };
}

macro_rules! crafter_recipe {
    (out: $output:expr; in: $input:expr; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([($input, 1)]), $output, 1, $time)
    };

    (out: $output:expr; in: $input:expr, $input_count:literal; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([($input, $input_count)]), $output, 1, $time)
    };

    (out: $output:expr, $output_count:literal; in: $input:expr; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([($input, 1)]), $output, $output_count, $time),
    };

    (out: $output:expr, $output_count:literal; in: $input:expr, $input_count:literal; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([($input, $input_count)]), $output, $output_count, $time)
    };

    (out: $output:expr, $output_count:literal; in: $(($input:expr, $input_count:literal)),+; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([
            $(($input, $input_count),)+
        ]), $output, $output_count, $time)
    };

    (out: $output:expr; in: $(($input:expr, $input_count:literal)),+; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([
            $(($input, $input_count),)+
        ]), $output, 1, $time)
    };

    (out: $output:expr, $output_count:literal; in: $($input:expr),+; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([
            $(($input, 1),)+
        ]), $output, $output_count, $time)
    };

    (out: $output:expr; in: $($input:expr),+; time: $time:expr) => {
        CrafterRecipe::new(HashMap::from([
            $(($input, 1),)+
        ]), $output, 1, $time)
    };
}

fn startup(mut commands: Commands) {
    commands.insert_resource(CrafterRecipes(vec![
        crafter_recipe!(out: Item::IronPlate; in: Item::IronIngot, 2; time: 7),
        crafter_recipe!(out: Item::CopperPlate; in: Item::CopperIngot, 2; time: 7),
        crafter_recipe!(out: Item::Gear; in: Item::IronPlate; time: 10),
    ]));

    commands.insert_resource(FurnaceRecipes(vec![
        furnace_recipe!(out: Item::CopperIngot; in: Item::RawCopper; time: 10),
        furnace_recipe!(out: Item::IronIngot; in: Item::RawIron; time: 10),
        furnace_recipe!(out: Item::Steel; in: Item::IronIngot; time: 50),
    ]));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
