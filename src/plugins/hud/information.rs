use bevy::prelude::*;

// MARK: Components

#[derive(Component)]
pub struct InformationText;

// MARK: Systems

/// Spawn the text
pub fn setup(mut commands: Commands) {
    // Spawn the text providing additional information
    commands.spawn((
        Text::new(include_str!("information.txt")),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        InformationText,
    ));
}

/// Remove the text
pub fn cleanup(mut commands: Commands, information_text: Single<Entity, With<InformationText>>) {
    commands.entity(information_text.entity()).despawn();
}
