use bevy::{
    color::palettes::tailwind::{GRAY_400, GRAY_500},
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

pub fn spawn_recipe_screen(
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

    let recipe_row_scenes = recipe_rows
        .into_iter()
        .map(|(recipe_text, is_current_recipe, recipe_button)| {
            bsn! {
                Node {
                    height: px(LINE_HEIGHT),
                    padding: {px(5).vertical()},
                    align_items: AlignItems::Center,
                }
                Text::new(recipe_text.clone())
                BackgroundColor({
                    if is_current_recipe {
                        GRAY_500
                    } else {
                        GRAY_400
                    }
                })
                template_value(recipe_button)
                Button
                Pickable {
                    should_block_lower: false,
                }
            }
        })
        .collect::<Vec<_>>();

    let scene = bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            Node {
                width: px(600),
                height: px(300),
                justify_content: JustifyContent::SpaceBetween,
                column_gap: px(10),
            }
            BackgroundColor(GRAY_500)
            Children [
                (
                    Node {
                        padding: px(5),
                        flex_direction: FlexDirection::Column,
                        width: percent(50),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Scroll
                        },
                    }
                    BackgroundColor(GRAY_400)
                    Children [{recipe_row_scenes}]
                ),
                (
                    Node {
                        width: percent(50),
                        padding: px(5),
                        height: auto(),
                    }
                    BackgroundColor(GRAY_400)
                    Children [
                        Text::new("")
                        RecipeDetailText
                    ]
                )
            ],
        ]

        RecipeScreen
    };

    commands.spawn_scene(scene);
}
