//! [TODO] Change this so every machine can define what happens on click itself
//! It's currently hardcoded because it needs to be

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::plugins::{
    crafting::recipe_types::Recipe,
    menu::{
        despawn_screen,
        game_menus::{
            GameMenuState,
            recipe_menu::{
                create_recipe_screen::create_recipe_screen, deselect_machine::deselect_machine,
                update_recipe_screen::update_recipe_screen,
            },
        },
    },
};

pub mod create_recipe_screen;
pub mod deselect_machine;
pub mod update_recipe_screen;

// MARK: Constants

const LINE_HEIGHT: f32 = 28.0;

// MARK: Plugin
pub struct RecipeMenuPlugin;

impl Plugin for RecipeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::Recipe), create_recipe_screen)
            .add_systems(
                Update,
                (update_recipe_screen, update_scroll_position)
                    .run_if(in_state(GameMenuState::Recipe)),
            )
            .add_systems(
                OnExit(GameMenuState::Recipe),
                (despawn_screen::<RecipeScreen>, deselect_machine),
            );
    }
}

// MARK: Components

#[derive(Component)]
pub struct RecipeScreen;

#[derive(Component)]
pub struct RecipeDetailText;

#[derive(Component, Deref)]
pub struct RecipeButton(Recipe);

// MARK: States

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
