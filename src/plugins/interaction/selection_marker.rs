use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    MouseCoordinates,
    machines::Machine,
    plugins::{
        RenderLayer,
        interaction::SelectionMarker,
        world::{MAP_SIZE, MAP_TYPE, TILE_SIZE},
    },
};

pub fn update_selection_marker(
    cursor_position: Res<MouseCoordinates>,
    selection_marker: Single<(&mut Transform, &mut Visibility), With<SelectionMarker>>,
    machine_tiles: Query<(&Machine, &TilePos)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let cursor_position = TilePos::new(cursor_position.x, cursor_position.y);
    let (mut selection_marker_transform, mut visibility) = selection_marker.into_inner();

    // Get the machine directly under the cursor
    let machine_under_cursor = machine_tiles
        .iter()
        .find(|&(_, &tile_pos)| tile_pos == cursor_position)
        .map(|(machine, _)| machine);

    match machine_under_cursor {
        // Check if the machine currently under the cursor is selectable
        Some(machine_under_cursor)
            if machine_under_cursor.machine_type.is_selectable()
                && !mouse_buttons.pressed(MouseButton::Right) =>
        {
            *visibility = Visibility::Visible;

            // Set the marker's transform to the current cursor position
            selection_marker_transform.translation = cursor_position
                .center_in_world(
                    &MAP_SIZE,
                    &TILE_SIZE.into(),
                    &TILE_SIZE,
                    &MAP_TYPE,
                    &TilemapAnchor::Center,
                )
                .extend(RenderLayer::SelectionMarker.into());
        }
        _ => {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn setup_selection_marker(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite::from_image(asset_server.load("selection.png")),
        Transform::from_xyz(0., 0., RenderLayer::SelectionMarker.into()),
        Visibility::Hidden,
        SelectionMarker,
    ));
}

pub fn despawn_selection_marker(
    mut commands: Commands,
    selection_marker: Single<Entity, With<SelectionMarker>>,
) {
    commands.entity(selection_marker.entity()).despawn();
}
