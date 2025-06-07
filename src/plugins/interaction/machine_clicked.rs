//! [TODO] Change this so every machine can define what happens on click itself
//! It's currently hardcoded because it needs to be

use bevy::{
    color::palettes::css::{BLACK, GRAY, LIGHT_GRAY},
    ecs::spawn::SpawnWith,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::plugins::crafting::{CrafterRecipes, recipe_types::CrafterRecipe};

#[derive(Component)]
pub struct RecipeScreen;

#[derive(Component)]
pub struct RecipeDetailText;

#[derive(Component, Deref)]
pub struct RecipeButton(CrafterRecipe);

#[allow(unused)]
pub fn update_recipe_screen(
    mut recipe_detail_text: Single<&mut Text, With<RecipeDetailText>>,
    interaction_query: Query<(&Interaction, &RecipeButton), (With<Button>)>,
) {
    let mut is_nothing_hovered = true;

    for (interaction, recipe) in interaction_query {
        match interaction {
            Interaction::Pressed => todo!(),
            Interaction::Hovered => {
                let mut ingredients_list = String::new();

                for (ingredient, ingredient_count) in &recipe.ingredients {
                    ingredients_list
                        .push_str(format!("- {ingredient_count}x {ingredient}\n").as_str());
                }

                ***recipe_detail_text = format!(
                    "{}\n\nIngredients:\n{ingredients_list}\nCrafting Time: {} ticks",
                    recipe.output_item, recipe.crafting_time,
                );

                is_nothing_hovered = false;
            }
            Interaction::None => {}
        }
    }

    if is_nothing_hovered {
        ***recipe_detail_text = String::new();
    }
}

pub fn create_recipe_screen(mut commands: Commands, recipes: Res<CrafterRecipes>) {
    let recipes = recipes.clone();

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
                    Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                        // Spawn the recipe buttons
                        for recipe in recipes {
                            parent.spawn((
                                Node {
                                    min_height: Val::Px(LINE_HEIGHT),
                                    padding: UiRect::vertical(Val::Px(5.0)),
                                    ..default()
                                },
                                Text::new(format!(
                                    "{}x {}",
                                    recipe.output_count, recipe.output_item
                                )),
                                TextColor(BLACK.into()),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                                RecipeButton(recipe),
                                Button,
                            ));
                        }
                    }))
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

pub fn despawn_recipe_screen(
    mut commands: Commands,
    recipe_screen: Single<Entity, With<RecipeScreen>>,
) {
    commands.entity(recipe_screen.entity()).despawn();
}

const LINE_HEIGHT: f32 = 21.0;

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
