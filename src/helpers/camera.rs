use bevy::{
    input::{
        ButtonInput,
        mouse::{MouseScrollUnit, MouseWheel},
    },
    prelude::*,
};

pub fn movement(
    mut camera: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut evr_scroll: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut zoom_add = 0.0;

    for event in evr_scroll.read() {
        match event.unit {
            MouseScrollUnit::Pixel => {
                if camera.single().0.scale >= 0.1 {
                    zoom_add += event.y * 0.1;
                }
            }
            MouseScrollUnit::Line => {
                if camera.single().0.scale >= 0.1 {
                    zoom_add += event.y * 5.0;
                }
            }
        }
    }

    camera.single_mut().0.scale += zoom_add * time.delta_secs();

    if camera.single().0.scale < 0.1 {
        camera.single_mut().0.scale = 0.1;
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

    translation =
        translation.normalize_or_zero() * 120.0 * camera.single().0.scale * time.delta_secs();

    camera.single_mut().1.translation += translation.extend(0.0);
}
