//! [TODO] Change this so every machine can define what happens on click itself
//! It's currently hardcoded because it needs to be

use bevy::{
    color::palettes::css::{BLACK, GRAY, LIGHT_GRAY},
    ecs::spawn::SpawnWith,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;

use crate::{MouseCoordinates, machines::Machine, plugins::interaction::RecipeScreen};

#[allow(unused)]
pub fn update_recipe_screen(
    cursor_position: Res<MouseCoordinates>,
    machine_tiles: Query<(&Machine, &TilePos)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    // todo!()
}

pub fn create_recipe_screen(mut commands: Commands) {
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
                width: Val::Px(200.0),
                height: Val::Px(100.0),
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
                        row_gap: Val::Px(10.0),
                        width: Val::Percent(50.0),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Scroll
                        },
                        ..default()
                    },
                    BackgroundColor(LIGHT_GRAY.into()),
                    Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                        for i in 0..5000 {
                            parent.spawn((
                                Node {
                                    min_height: Val::Px(LINE_HEIGHT),
                                    ..default()
                                },
                                Text::new(format!("#{}", i + 1)),
                                TextColor(BLACK.into()),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
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
                    children![(Text::new("Hi"), TextColor(BLACK.into()))]
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
