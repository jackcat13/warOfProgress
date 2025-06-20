use bevy::prelude::*;

use crate::camera::MainCamera;

use super::world_components::{NewPositions, SelectChild, Selected, UnitId};

const SPEED: f32 = 100.0;

pub fn highlight_selected_units(
    query: Query<(&UnitId, &mut Visibility), With<SelectChild>>,
    selected: Res<Selected>,
) {
    for (id, mut visibility) in query {
        *visibility = if selected.contains(id) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}

pub fn check_movement_on_right_click(
    query: Query<&UnitId, With<Pickable>>,
    selected: Res<Selected>,
    window: Single<&Window>,
    camera_single: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut new_positions: ResMut<NewPositions>,
) {
    if !buttons.just_released(MouseButton::Right) {
        return;
    }
    let (camera, camera_transform) = (camera_single.0, camera_single.1);
    for unit_id in query {
        if !selected.contains(unit_id) {
            continue;
        }
        let Some(Ok(world_position)) = window
            .cursor_position()
            .map(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        else {
            continue;
        };
        new_positions
            .positions
            .insert(unit_id.clone(), world_position);
    }
}

pub fn move_units(
    query: Query<(&UnitId, &mut Transform), With<Pickable>>,
    new_positions: ResMut<NewPositions>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (unit_id, mut transform) in query {
        let Some(target_position) = new_positions.get(unit_id) else {
            continue;
        };
        let mut current_position = transform.translation.xy();
        if current_position.x < target_position.x {
            current_position.x += SPEED * delta;
            if current_position.x > target_position.x {
                current_position.x = target_position.x;
            }
        }
        if current_position.x > target_position.x {
            current_position.x -= SPEED * delta;
            if current_position.x < target_position.x {
                current_position.x = target_position.x;
            }
        }
        if current_position.y < target_position.y {
            current_position.y += SPEED * delta;
            if current_position.y > target_position.y {
                current_position.y = target_position.y;
            }
        }
        if current_position.y > target_position.y {
            current_position.y -= SPEED * delta;
            if current_position.y < target_position.y {
                current_position.y = target_position.y;
            }
        }
        transform.translation.x = current_position.x;
        transform.translation.y = current_position.y;
    }
}
