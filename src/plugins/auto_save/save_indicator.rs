use bevy::prelude::*;

use crate::plugins::auto_save::{SaveIndicator, SaveIndicatorTimer};

pub fn setup_save_indicator(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            width: Val::Vw(100.0),
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Visibility::Hidden,
        Text::new("Saved!"),
        SaveIndicator,
    ));
}

pub fn update_save_indicator(
    mut save_indicator_visibility: Single<&mut Visibility, With<SaveIndicator>>,
    mut save_indicator_timer: ResMut<SaveIndicatorTimer>,
    time: Res<Time>,
) {
    if save_indicator_timer.tick(time.delta()).just_finished() {
        **save_indicator_visibility = Visibility::Hidden;
    }
}
