use bevy::prelude::*;

use crate::camera::MainCamera;

pub fn get_mouse_position(
    window: Single<&Window>,
    camera: Single<&Camera, With<MainCamera>>,
    camera_transform: Single<&GlobalTransform, With<MainCamera>>,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(&camera_transform, cursor).ok())
}

