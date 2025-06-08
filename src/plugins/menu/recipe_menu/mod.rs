//! [TODO] Change this so every machine can define what happens on click itself
//! It's currently hardcoded because it needs to be

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::plugins::crafting::recipe_types::Recipe;

pub mod create_recipe_screen;
pub mod deselect_machine;
pub mod update_recipe_screen;

#[derive(Component)]
pub struct RecipeScreen;

#[derive(Component)]
pub struct RecipeDetailText;

#[derive(Component, Deref)]
pub struct RecipeButton(Recipe);

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
