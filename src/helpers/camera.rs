use bevy::{
    input::{
        ButtonInput,
        mouse::{MouseScrollUnit, MouseWheel},
    },
    prelude::*,
};

use crate::plugins::world::MAP_SIZE;

pub fn movement(
    mut camera: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut evr_scroll: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut zoom_add = 0.0;
    let (mut projection, mut camera) = camera.single_mut();

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

    if camera.translation.x < 0.0 {
        camera.translation.x = 0.0
    } else if camera.translation.x > MAP_SIZE.x as f32 {
        camera.translation.x = MAP_SIZE.x as f32;
    }

    if camera.translation.y < 0.0 {
        camera.translation.y = 0.0
    } else if camera.translation.y > MAP_SIZE.y as f32 {
        camera.translation.y = MAP_SIZE.y as f32;
    }
}
