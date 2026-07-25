use bevy::prelude::*;

// MARK: Components

#[derive(Component, Clone, Copy, Default)]
pub struct InformationText;

// MARK: Systems

/// Spawn the text
pub fn information_hud() -> impl Scene {
    bsn! {
        Text::new(include_str!("information.txt"))
        TextLayout::justify(Justify::Left)
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(5),
        }
        InformationText
    }
}

/// Remove the text
pub fn cleanup(mut commands: Commands, information_text: Single<Entity, With<InformationText>>) {
    commands.entity(information_text.entity()).despawn();
}
