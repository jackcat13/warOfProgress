use bevy::math::Vec2;

pub fn is_entity_in_hitbox(
    test_position: Vec2,
    box_position: Vec2,
    box_size: Vec2,
) -> bool {
    let box_position = Vec2 { x: box_position.x - box_size.x / 2., y: box_position.y - box_size.y / 2. };
    test_position.x >= box_position.x
    && test_position.x <= box_position.x + box_size.x
    && test_position.y >= box_position.y
    && test_position.y <= box_position.y + box_size.y
}

pub fn has_entity_reached_target(
    test_position: Vec2,
    target_position: Vec2,
    distance: f32,
) -> bool {
    f32::abs(test_position.x - target_position.x) < distance &&
    f32::abs(test_position.y - target_position.y) < distance
}
