use bevy::prelude::*;

use crate::plugins::save::{SaveIndicator, SaveIndicatorTimer};

pub fn save_indicator() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: px(80),
            width: vw(100),
        }
        TextLayout::justify(Justify::Center)
        Visibility::Hidden
        Text::new("Saving...")
        SaveIndicator
    }
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
