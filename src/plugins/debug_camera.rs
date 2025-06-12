use bevy::{
    input::{
        ButtonInput,
        mouse::{MouseScrollUnit, MouseWheel},
    },
    prelude::*,
};

use crate::{
    game_save_types::LoadedGameSave,
    plugins::{
        interaction::{can_interact_with_world, game_not_paused},
        menu::GameState,
    },
};

pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), startup)
            // We want the player to still be able to move around when in a menu, but he shouldn't be able to move
            .add_systems(Update, zoom.run_if(can_interact_with_world))
            .add_systems(Update, movement.run_if(game_not_paused))
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
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut projection, mut camera) = camera.into_inner();
    let Projection::Orthographic(projection) = &mut *projection else {
        return;
    };

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

fn zoom(
    camera_projection: Single<&mut Projection>,
    mut evr_scroll: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Projection::Orthographic(projection) = &mut *camera_projection.into_inner() else {
        return;
    };

    let mut zoom_add = 0.0;

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

    // Reset zoom when space is pressed
    if keys.just_pressed(KeyCode::Space) {
        projection.scale = 1.0;
    }

    if projection.scale < 0.1 {
        projection.scale = 0.1;
    }
}

fn cleanup(camera: Single<(&mut Projection, &mut Transform)>) {
    let (mut projection, mut camera) = camera.into_inner();
    let Projection::Orthographic(projection) = &mut *projection else {
        return;
    };

    projection.scale = 1.0;
    camera.translation = Vec3::ZERO;
}
