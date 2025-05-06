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
    mut coordinates_text: Query<&mut Text, With<CoordinatesText>>,
    mouse_coords: Res<MouseCoordinates>,
) {
    for mut span in coordinates_text.iter_mut() {
        **span = format!("X: {}, Y: {}", mouse_coords.x, mouse_coords.y);
    }
}
