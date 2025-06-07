#![allow(unused)] // [TODO] Remove once this is all implemented

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

pub fn startup(mut commands: Commands) {
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

    commands.init_resource::<FurnaceRecipes>();
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
