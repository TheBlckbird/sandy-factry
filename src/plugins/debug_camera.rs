use bevy::{
    input::{
        ButtonInput,
        mouse::{MouseScrollUnit, MouseWheel},
    },
    prelude::*,
};

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(
    camera: Single<(&mut Projection, &mut Transform)>,
    mut evr_scroll: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut zoom_add = 0.0;
    let (mut projection, mut camera) = camera.into_inner();
    let Projection::Orthographic(projection) = &mut *projection else {
        return;
    };

    for event in evr_scroll.read() {
        match event.unit {
            MouseScrollUnit::Pixel => {
                if projection.scale >= 0.1 {
                    zoom_add += event.y * 0.1;
                }
            }
            MouseScrollUnit::Line => {
                if projection.scale >= 0.1 {
                    zoom_add += event.y * 5.0;
                }
            }
        }
    }

    projection.scale += zoom_add * time.delta_secs();

    if projection.scale < 0.1 {
        projection.scale = 0.1;
    }

    let mut translation = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        translation.y += 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        translation.x += 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        translation.y -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        translation.x -= 1.0;
    }

    translation = translation.normalize_or_zero() * 120.0 * projection.scale * time.delta_secs();

    camera.translation += translation.extend(0.0);
}
