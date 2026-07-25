use bevy::{
    color::palettes::tailwind::{GRAY_400, GRAY_500},
    ecs::spawn::SpawnWith,
    prelude::*,
};

use crate::{
    content::{
        machine_types::Machine,
        machines::{crafter::Crafter, furnace::Furnace},
    },
    plugins::{
        crafting::{CrafterRecipes, FurnaceRecipes, recipe_types::Recipe},
        interaction::SelectedMachine,
        menu::game_menus::recipe_menu::{
            LINE_HEIGHT, RecipeButton, RecipeDetailText, RecipeScreen,
        },
    },
};

pub fn create_recipe_screen(
    mut commands: Commands,
    crafter_recipes: Res<CrafterRecipes>,
    furnace_recipes: Res<FurnaceRecipes>,
    selected_machine: Single<&Machine, With<SelectedMachine>>,
) {
    let mut recipe_rows = Vec::new();

    if let Some(crafter) = selected_machine
        .machine_type
        .as_ref()
        .as_any()
        .downcast_ref::<Crafter>()
    {
        for recipe in crafter_recipes.iter() {
            recipe_rows.push((
                recipe.output_item.to_string(),
                crafter
                    .current_recipe
                    .clone()
                    .map(|current_recipe| current_recipe == *recipe)
                    .is_some_and(|value| value),
                RecipeButton(Recipe::Crafter(recipe.clone())),
            ));
        }
    } else if let Some(furnace) = selected_machine
        .machine_type
        .as_ref()
        .as_any()
        .downcast_ref::<Furnace>()
    {
        for recipe in furnace_recipes.iter() {
            recipe_rows.push((
                recipe.output_item.0.to_string(),
                furnace
                    .current_recipe
                    .map(|current_recipe| current_recipe == *recipe)
                    .is_some_and(|value| value),
                RecipeButton(Recipe::Furnace(*recipe)),
            ));
        }
    }

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                width: px(600),
                height: px(300),
                justify_content: JustifyContent::SpaceBetween,
                column_gap: px(10),
                ..default()
            },
            BackgroundColor(GRAY_500.into()),
            children![
                (
                    Node {
                        padding: px(5).all(),
                        flex_direction: FlexDirection::Column,
                        width: percent(50),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Scroll
                        },
                        ..default()
                    },
                    BackgroundColor(GRAY_400.into()),
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        for (recipe_text, is_current_recipe, recipe_button) in recipe_rows {
                            parent.spawn((
                                Node {
                                    height: px(LINE_HEIGHT),
                                    padding: px(5).vertical(),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                Text::new(recipe_text.to_string()),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                                if is_current_recipe {
                                    BackgroundColor(GRAY_500.into())
                                } else {
                                    BackgroundColor(GRAY_400.into())
                                },
                                recipe_button,
                                Button,
                            ));
                        }
                    }))
                ),
                (
                    Node {
                        width: percent(50),
                        padding: px(5).all(),
                        height: auto(),
                        ..default()
                    },
                    BackgroundColor(GRAY_400.into()),
                    children![(Text::new(""), RecipeDetailText)]
                )
            ],
        )],
        RecipeScreen,
    ));
}
