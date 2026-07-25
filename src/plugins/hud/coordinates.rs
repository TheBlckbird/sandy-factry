use bevy::prelude::*;

use crate::MouseCoordinates;

#[derive(Component, Clone, Copy, Default)]
pub struct CoordinatesText;

pub fn coordinates_hud() -> impl Scene {
    bsn! {
        Text::new("X: ---, Y: ---")
        TextLayout::justify(Justify::Center)
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(5),
        }
        CoordinatesText
    }
}

/// Update the coordinates text
pub fn update_coordinates(
    mut coordinates_text: Single<&mut Text, With<CoordinatesText>>,
    mouse_coords: Res<MouseCoordinates>,
) {
    coordinates_text.0 = format!("X: {}, Y: {}", mouse_coords.x, mouse_coords.y);
}

pub fn cleanup(mut commands: Commands, coordinates_text: Single<Entity, With<CoordinatesText>>) {
    commands.entity(coordinates_text.entity()).despawn();
}
