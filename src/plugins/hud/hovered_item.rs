use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{MouseCoordinates, content::items::Item};

// MARK: Components

#[derive(Component)]
pub struct HoveredItemText;

// MARK: Systems

pub fn setup(mut commands: Commands) {
    // Spawn text for the hovered items
    commands.spawn((
        Text::new(""),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        HoveredItemText,
    ));
}

/// Update the hovered items text
pub fn update_hovered_item_text(
    mut hovered_item_text: Single<&mut Text, With<HoveredItemText>>,
    mouse_coords: Res<MouseCoordinates>,
    item_tiles: Query<(&Item, &TilePos)>,
) {
    // Get the item under the current cursor position
    hovered_item_text.0 = match item_tiles
        .iter()
        .find(|&(_, &tile_pos)| mouse_coords.as_tile_pos() == tile_pos)
    {
        // Set the label to the item name if there is an item under the cursor
        Some((item, _)) => item.to_string(),

        // If there is no item under the cursor, just set it to a blank string
        None => String::new(),
    };
}

pub fn cleanup(mut commands: Commands, hovered_item_text: Single<Entity, With<HoveredItemText>>) {
    commands.entity(hovered_item_text.entity()).despawn();
}
