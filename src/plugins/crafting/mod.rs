#![allow(unused)] // [TODO] Remove once this is all implemented

use bevy::prelude::*;
use recipe_types::{CrafterRecipe, FurnaceRecipe};

use super::menu::GameState;

pub mod recipe_types;

#[derive(Debug, Resource, Default)]
pub struct CrafterRecipes(Vec<CrafterRecipe>);

#[derive(Debug, Resource, Default)]
pub struct FurnaceRecipes(Vec<FurnaceRecipe>);

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), startup)
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

fn startup(mut commands: Commands) {
    commands.init_resource::<FurnaceRecipes>();
    commands.init_resource::<FurnaceRecipes>();
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<CrafterRecipes>();
    commands.remove_resource::<FurnaceRecipes>();
}
