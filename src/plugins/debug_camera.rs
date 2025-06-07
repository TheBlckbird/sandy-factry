use bevy::{input::ButtonInput, prelude::*};

use crate::game_save_types::LoadedGameSave;

use super::menu::GameState;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), startup)
            .add_systems(Update, movement.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

fn startup(game_save: Res<LoadedGameSave>, camera: Single<&mut Transform, With<Camera2d>>) {
    if let Some(game_save) = &**game_save {
        camera.into_inner().translation = game_save.camera_translation;
    }
}

fn movement(
    camera: Single<(&mut Projection, &mut Transform)>,
    // mut evr_scroll: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // let mut zoom_add = 0.0;
    let (mut projection, mut camera) = camera.into_inner();
    let Projection::Orthographic(projection) = &mut *projection else {
        return;
    };

    // for event in evr_scroll.read() {
    //     match event.unit {
    //         MouseScrollUnit::Pixel => {
    //             if projection.scale >= 0.1 {
    //                 zoom_add += event.y * 0.1;
    //             }
    //         }
    //         MouseScrollUnit::Line => {
    //             if projection.scale >= 0.1 {
    //                 zoom_add += event.y * 5.0;
    //             }
    //         }
    //     }
    // }

    // projection.scale += zoom_add * time.delta_secs();

    // if projection.scale < 0.1 {
    //     projection.scale = 0.1;
    // }

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

fn cleanup(camera: Single<(&mut Projection, &mut Transform)>) {
    let (mut projection, mut camera) = camera.into_inner();
    let Projection::Orthographic(projection) = &mut *projection else {
        return;
    };

    projection.scale = 1.0;
    camera.translation = Vec3::ZERO;
}
