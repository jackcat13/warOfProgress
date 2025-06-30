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


pub fn is_mouse_in_hitbox(
    mouse_position: Vec2,
    box_position: Vec2,
    box_size: Vec2,
) -> bool {
    let box_position = Vec2 { x: box_position.x - box_size.x / 2., y: box_position.y - box_size.y / 2. };
    mouse_position.x >= box_position.x
    && mouse_position.x <= box_position.x + box_size.x
    && mouse_position.y >= box_position.y
    && mouse_position.y <= box_position.y + box_size.y
}
