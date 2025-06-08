use bevy::prelude::*;

use crate::{
    machines::{Machine, crafter::Crafter, furnace::Furnace},
    plugins::{
        crafting::recipe_types::Recipe,
        interaction::SelectedMachine,
        menu::{
            GameMenuState,
            recipe_menu::{RecipeButton, RecipeDetailText},
        },
    },
};

pub fn update_recipe_screen(
    mut commands: Commands,
    mut recipe_detail_text: Single<&mut Text, With<RecipeDetailText>>,
    interaction_query: Query<(&Interaction, &RecipeButton), With<Button>>,
    mut selected_machine: Single<(Entity, &mut Machine), With<SelectedMachine>>,
    mut game_menu_state: ResMut<NextState<GameMenuState>>,
) {
    let mut is_nothing_hovered = true;
    let mut is_nothing_pressed = true;

    for (interaction, recipe_button) in interaction_query {
        match interaction {
            Interaction::Hovered => {
                match &recipe_button.0 {
                    Recipe::Crafter(crafter_recipe) => {
                        let mut ingredients_list = String::new();

                        for (ingredient, ingredient_count) in &crafter_recipe.ingredients {
                            ingredients_list
                                .push_str(format!("- {ingredient_count}x {ingredient}\n").as_str());
                        }

                        ***recipe_detail_text = format!(
                            "{}\n\nIngredients:\n{ingredients_list}\nCrafting Time: {} ticks",
                            crafter_recipe.output_item, crafter_recipe.crafting_time,
                        );
                    }
                    Recipe::Furnace(furnace_recipe) => {
                        ***recipe_detail_text = format!(
                            "{}\n\nIngredients:\n- {}x {}\nBurn Time: {} ticks",
                            furnace_recipe.output_item.0,
                            furnace_recipe.ingredient.1,
                            furnace_recipe.ingredient.0,
                            furnace_recipe.burn_time,
                        );
                    }
                }

                is_nothing_hovered = false;
            }
            Interaction::Pressed => match &recipe_button.0 {
                Recipe::Crafter(crafter_recipe) => {
                    if let Some(crafter) = selected_machine
                        .1
                        .machine_type
                        .as_mut()
                        .as_any_mut()
                        .downcast_mut::<Crafter>()
                    {
                        crafter.current_recipe = Some(crafter_recipe.clone());
                        is_nothing_pressed = false;
                    }
                }
                Recipe::Furnace(furnace_recipe) => {
                    if let Some(furnace) = selected_machine
                        .1
                        .machine_type
                        .as_mut()
                        .as_any_mut()
                        .downcast_mut::<Furnace>()
                    {
                        furnace.current_recipe = Some(*furnace_recipe);
                        is_nothing_pressed = false;
                    }
                }
            },
            Interaction::None => {}
        }
    }

    if is_nothing_hovered {
        ***recipe_detail_text = String::new();
    }

    if !is_nothing_pressed {
        game_menu_state.set(GameMenuState::Hidden);
        commands
            .entity(selected_machine.0)
            .remove::<SelectedMachine>();
    }
}
