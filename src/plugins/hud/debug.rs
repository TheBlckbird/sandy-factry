use bevy::prelude::*;

use crate::MouseCoordinates;

use super::CoordinatesText;

pub fn setup(mut commands: Commands) {
    // Spawn text for coordinates
    commands.spawn((
        Text::new("X: ---, Y: ---"),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        CoordinatesText,
    ));
}

pub fn update_coordinates(
    mut coordinates_text: Single<&mut Text, With<CoordinatesText>>,
    mouse_coords: Res<MouseCoordinates>,
) {
    coordinates_text.0 = format!("X: {}, Y: {}", mouse_coords.x, mouse_coords.y);
}

pub fn cleanup(mut commands: Commands, coordinates_text: Single<Entity, With<CoordinatesText>>) {
    commands.entity(coordinates_text.entity()).despawn();
}
