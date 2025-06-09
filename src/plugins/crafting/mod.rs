use std::collections::HashMap;

use bevy::prelude::*;
use recipe_types::{CrafterRecipe, FurnaceRecipe};

use crate::content::items::Item;

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

fn startup(mut commands: Commands) {
    commands.insert_resource(CrafterRecipes(vec![
        // Basic Components
        crafter_recipe!(out: Item::Gear; in: Item::IronIngot, 2; time: 5),
        crafter_recipe!(out: Item::Wire, 2; in: Item::CopperIngot; time: 5),
        crafter_recipe!(out: Item::ReinforcedSteel; in: (Item::IronIngot, 1), (Item::Steel, 2); time: 50),

        // Advanced Components
        crafter_recipe!(out: Item::ElectricalCircuit; in: (Item::Wire, 3), (Item::CopperIngot, 1); time: 50),
        crafter_recipe!(out: Item::MicroProcessor; in: (Item::ElectricalCircuit, 2), (Item::IronIngot, 1); time: 75),
        crafter_recipe!(out: Item::Motor; in: (Item::Steel, 2), (Item::Gear, 3); time: 100),
        crafter_recipe!(out: Item::Battery; in: (Item::CopperIngot, 2), (Item::Coal, 1); time: 50),
        crafter_recipe!(out: Item::Engine; in: (Item::Motor, 2), (Item::Gear, 2); time: 100),
        
        // Helicopter Components
        crafter_recipe!(out: Item::RotorBlade; in: Item::Steel, 3; time: 50),
        crafter_recipe!(out: Item::Propeller; in: Item::RotorBlade, 4; time: 75),
        crafter_recipe!(out: Item::BigPropeller; in: (Item::RotorBlade, 4), (Item::Gear, 2); time: 100),
        crafter_recipe!(out: Item::Hull; in: (Item::ReinforcedSteel, 2), (Item::Steel, 2); time: 100),
        crafter_recipe!(out: Item::ControlModule; in: (Item::MicroProcessor, 2), (Item::Motor, 1); time: 200),
        crafter_recipe!(out: Item::HelicopterFrame; in: (Item::Hull, 3), (Item::Gear, 4); time: 200),
        crafter_recipe!(out: Item::Helicopter; in:
            Item::HelicopterFrame,
            Item::Engine,
            Item::Propeller,
            Item::BigPropeller,
            Item::ControlModule;
        time: 600),
    ]));

    commands.insert_resource(FurnaceRecipes(vec![
        furnace_recipe!(out: Item::CopperIngot; in: Item::RawCopper; time: 7),
        furnace_recipe!(out: Item::IronIngot; in: Item::RawIron; time: 7),
        furnace_recipe!(out: Item::Steel; in: Item::IronIngot, 2; time: 25),
    ]));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
