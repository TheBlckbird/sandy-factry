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

fn startup(mut commands: Commands) {
    commands.insert_resource(CrafterRecipes(vec![
        CrafterRecipe::new(HashMap::from([(Item::IronIngot, 2)]), Item::IronPlate, 1, 7),
        CrafterRecipe::new(
            HashMap::from([(Item::CopperIngot, 2)]),
            Item::CopperPlate,
            1,
            7,
        ),
        CrafterRecipe::new(HashMap::from([(Item::IronPlate, 1)]), Item::Gear, 1, 10),
    ]));

    commands.insert_resource(FurnaceRecipes(vec![
        FurnaceRecipe::new((Item::CopperIngot, 1), (Item::RawCopper, 1), 10),
        FurnaceRecipe::new((Item::IronIngot, 1), (Item::RawIron, 1), 10),
        FurnaceRecipe::new((Item::Steel, 1), (Item::IronPlate, 1), 50),
    ]));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
