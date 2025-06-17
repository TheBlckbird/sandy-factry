use std::collections::HashMap;

use bevy::prelude::*;
use recipe_types::{CrafterRecipe, FurnaceRecipe};

use crate::{content::items::ItemType, plugins::menu::GameState};

pub mod recipe_types;

// MARK: Plugin
pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), add_recipes)
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

// MARk: Resources

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct CrafterRecipes(Vec<CrafterRecipe>);

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct FurnaceRecipes(Vec<FurnaceRecipe>);

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
        CrafterRecipe::new(HashMap::from([($input, 1)]), $output, $output_count, $time)
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

// MARK: Systems

/// Add all recipes
fn add_recipes(mut commands: Commands) {
    commands.insert_resource(CrafterRecipes(vec![
        // Basic Components
        crafter_recipe!(out: ItemType::Gear; in: ItemType::IronIngot, 2; time: 5),
        crafter_recipe!(out: ItemType::Wire, 2; in: ItemType::CopperIngot; time: 5),
        crafter_recipe!(out: ItemType::ReinforcedSteel; in: (ItemType::IronIngot, 1), (ItemType::Steel, 2); time: 50),

        // Advanced Components
        crafter_recipe!(out: ItemType::ElectricalCircuit; in: (ItemType::Wire, 3), (ItemType::CopperIngot, 1); time: 50),
        crafter_recipe!(out: ItemType::MicroProcessor; in: (ItemType::ElectricalCircuit, 2), (ItemType::IronIngot, 1); time: 75),
        crafter_recipe!(out: ItemType::Motor; in: (ItemType::Steel, 2), (ItemType::Gear, 3); time: 100),
        crafter_recipe!(out: ItemType::Battery; in: (ItemType::CopperIngot, 2), (ItemType::Coal, 1); time: 50),
        crafter_recipe!(out: ItemType::Engine; in: (ItemType::Motor, 2), (ItemType::Gear, 2); time: 100),
        
        // Helicopter Components
        crafter_recipe!(out: ItemType::RotorBlade; in: ItemType::Steel, 3; time: 50),
        crafter_recipe!(out: ItemType::Propeller; in: ItemType::RotorBlade, 4; time: 75),
        crafter_recipe!(out: ItemType::BigPropeller; in: (ItemType::RotorBlade, 4), (ItemType::Gear, 2); time: 100),
        crafter_recipe!(out: ItemType::Hull; in: (ItemType::ReinforcedSteel, 2), (ItemType::Steel, 2); time: 100),
        crafter_recipe!(out: ItemType::ControlModule; in: (ItemType::MicroProcessor, 2), (ItemType::Motor, 1); time: 200),
        crafter_recipe!(out: ItemType::HelicopterFrame; in: (ItemType::Hull, 3), (ItemType::Gear, 4); time: 200),
        crafter_recipe!(out: ItemType::Helicopter; in:
            (ItemType::HelicopterFrame, 1),
            (ItemType::Engine, 1),
            (ItemType::Propeller, 1),
            (ItemType::BigPropeller, 1),
            (ItemType::ControlModule, 1),
            (ItemType::Battery, 2);
        time: 600),
    ]));

    commands.insert_resource(FurnaceRecipes(vec![
        furnace_recipe!(out: ItemType::CopperIngot; in: ItemType::RawCopper; time: 7),
        furnace_recipe!(out: ItemType::IronIngot; in: ItemType::RawIron; time: 7),
        furnace_recipe!(out: ItemType::Steel; in: ItemType::IronIngot, 2; time: 25),
    ]));
}

/// Remove all recipes
fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
