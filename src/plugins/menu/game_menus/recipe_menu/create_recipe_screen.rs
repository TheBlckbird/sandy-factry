use bevy::{
    color::palettes::css::{BLACK, GRAY, LIGHT_GRAY},
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
    let mut recipe_children: Option<Box<dyn FnOnce(&mut ChildSpawner) + Send + Sync>> = None;

    if let Some(crafter) = selected_machine
        .machine_type
        .as_ref()
        .as_any()
        .downcast_ref::<Crafter>()
    {
        let recipes = crafter_recipes.clone();
        let current_recipe = crafter.current_recipe.clone();

        recipe_children = Some(Box::new(move |parent: &mut ChildSpawner| {
            // Spawn the recipe buttons
            for recipe in recipes {
                parent.spawn((
                    Node {
                        min_height: Val::Px(LINE_HEIGHT),
                        padding: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    Text::new(format!(
                        "{}x {}{}",
                        recipe.output_count,
                        recipe.output_item,
                        if let Some(ref current_recipe) = current_recipe
                            && *current_recipe == recipe
                        {
                            " (current)"
                        } else {
                            ""
                        }
                    )),
                    TextColor(BLACK.into()),
                    Pickable {
                        should_block_lower: false,
                        ..default()
                    },
                    RecipeButton(Recipe::Crafter(recipe)),
                    Button,
                ));
            }
        }));
    } else if let Some(furnace) = selected_machine
        .machine_type
        .as_ref()
        .as_any()
        .downcast_ref::<Furnace>()
    {
        let recipes = furnace_recipes.clone();
        let current_recipe = furnace.current_recipe;

        recipe_children = Some(Box::new(move |parent: &mut ChildSpawner| {
            // Spawn the recipe buttons
            for recipe in recipes {
                parent.spawn((
                    Node {
                        min_height: Val::Px(LINE_HEIGHT),
                        padding: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    Text::new(format!(
                        "{}x {}{}",
                        recipe.output_item.1,
                        recipe.output_item.0,
                        if let Some(current_recipe) = current_recipe
                            && current_recipe == recipe
                        {
                            " (current)"
                        } else {
                            ""
                        }
                    )),
                    TextColor(BLACK.into()),
                    Pickable {
                        should_block_lower: false,
                        ..default()
                    },
                    RecipeButton(Recipe::Furnace(recipe)),
                    Button,
                ));
            }
        }));
    }

    if let Some(recipe_children) = recipe_children {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![(
                Node {
                    width: Val::Px(600.0),
                    height: Val::Px(300.0),
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(GRAY.into()),
                children![
                    (
                        Node {
                            padding: UiRect::all(Val::Px(5.0)),
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(50.0),
                            overflow: Overflow {
                                x: OverflowAxis::Hidden,
                                y: OverflowAxis::Scroll
                            },
                            ..default()
                        },
                        BackgroundColor(LIGHT_GRAY.into()),
                        Children::spawn(SpawnWith(recipe_children))
                    ),
                    (
                        Node {
                            width: Val::Percent(50.0),
                            padding: UiRect::all(Val::Px(5.0)),
                            height: Val::Auto,
                            ..default()
                        },
                        BackgroundColor(LIGHT_GRAY.into()),
                        children![(Text::new(""), TextColor(BLACK.into()), RecipeDetailText)]
                    )
                ],
            )],
            RecipeScreen,
        ));
    }
}
