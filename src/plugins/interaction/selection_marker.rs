use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    MouseCoordinates,
    machines::Machine,
    plugins::{
        RenderLayer,
        interaction::{SelectedMachine, SelectionMarker},
        menu::GameMenuState,
        world::{MAP_SIZE, MAP_TYPE, TILE_SIZE},
    },
};

pub fn update_selection_marker(
    mut commands: Commands,
    cursor_position: Res<MouseCoordinates>,
    selection_marker: Single<(&mut Transform, &mut Visibility), With<SelectionMarker>>,
    machine_tiles: Query<(Entity, &Machine, &TilePos)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut game_menu_state: ResMut<NextState<GameMenuState>>,
) {
    let cursor_position = TilePos::new(cursor_position.x, cursor_position.y);
    let (mut selection_marker_transform, mut selection_marker_visibility) =
        selection_marker.into_inner();

    // Get the machine directly under the cursor
    let machine_under_cursor = machine_tiles
        .iter()
        .find(|&(_, _, &tile_pos)| tile_pos == cursor_position)
        .map(|(entity, machine, _)| (entity, machine));

    match machine_under_cursor {
        // Check if the machine currently under the cursor is selectable
        Some((machine_entity, machine_under_cursor))
            if machine_under_cursor.machine_type.is_selectable()
                // If this check wasn't made, the marker would flicker when deleting multiple buildings at once
                && !mouse_buttons.pressed(MouseButton::Right) =>
        {
            *selection_marker_visibility = Visibility::Visible;

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

            // If this machine is clicked, add the marker component
            if mouse_buttons.just_pressed(MouseButton::Left) {
                commands.entity(machine_entity).insert(SelectedMachine);
                game_menu_state.set(GameMenuState::Recipe);
            }
        }
        _ => {
            *selection_marker_visibility = Visibility::Hidden;
        }
    }
}

pub fn hide_selection_marker(mut selection_marker: Single<&mut Visibility, With<SelectionMarker>>) {
    **selection_marker = Visibility::Hidden;
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
