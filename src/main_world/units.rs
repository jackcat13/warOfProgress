use bevy::prelude::*;

use crate::{
    camera::MainCamera,
    main_world::builds::resolve_building_from_asset,
    mouse::{get_mouse_position, is_mouse_in_hitbox},
};

use super::{
    hitbox::is_entity_in_hitbox,
    world_components::{
        Achievement, BuildTarget, Hitboxes, Moving, NewPositions, SelectChild, Selected, UnitId, Villager
    },
};

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
    movables: Query<(&UnitId, &mut Moving, &BuildTarget), With<Pickable>>,
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
    for (unit_id, mut moving, build_target) in movables {
        if build_target.id.is_some() {
            continue;
        }
        if !selected.contains(unit_id) {
            continue;
        }
        let Some(Ok(world_position)) = window
            .cursor_position()
            .map(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        else {
            continue;
        };
        new_positions.insert_or_update(unit_id, world_position);
        *moving = Moving(true);
    }
}

pub fn check_build_target_on_right_click(
    villagers: Query<(&UnitId, &mut BuildTarget), With<Villager>>,
    builds: Query<(&UnitId, &Transform, &Sprite), With<Achievement>>,
    selected: Res<Selected>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<&Camera, With<MainCamera>>,
    camera_transform: Single<&GlobalTransform, With<MainCamera>>,
    mut new_positions: ResMut<NewPositions>,
) {
    if !buttons.just_released(MouseButton::Right) {
        return;
    }
    let Some(mouse_position) = get_mouse_position(window, camera, camera_transform) else {
        return;
    };
    for (unit_id, mut build_target) in villagers {
        if !selected.contains(unit_id) {
            continue;
        }
        for (build_id, build_transform, build_sprite) in builds {
            let build_xy = build_transform.translation.xy();
            let build_size = build_sprite
                .custom_size
                .expect("BUG - Build sprite must have custom_size");
            if !is_mouse_in_hitbox(mouse_position, build_xy, build_size) {
                build_target.id = None;
                continue;
            }
            build_target.id = Some(build_id.clone());
            println!("{:?}", new_positions);
            let processed_position = Vec2 {
                x: build_xy.x - build_size.x / 2.,
                y: build_xy.y - build_size.y / 2.,
            };
            new_positions.insert_or_update(unit_id, processed_position);
        }
    }
}

pub fn process_building_builds(
    villagers: Query<(&mut BuildTarget, &Moving), With<Villager>>,
    mut builds: Query<(&UnitId, &mut Achievement, &mut Sprite), With<Achievement>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (mut build_target, moving) in villagers {
        if moving.0 {
            continue;
        }
        let Some(build_target_id) = &build_target.id else {
            continue;
        };
        let Some((_, mut achievement, mut sprite)) = builds
            .iter_mut()
            .find(|(build_id, _, _)| *build_id == build_target_id)
        else {
            continue;
        };
        achievement.progress += 10. * time.delta_secs();
        println!("Progress {:?}", achievement.progress);
        if achievement.progress >= 100. {
            build_target.id = None;
            let building_specs = resolve_building_from_asset(&mut sprite.image);
            sprite.image = asset_server.load(building_specs.resolve_finished_asset_path());
        }
    }
}

pub fn process_hitboxes(
    movables: Query<(&UnitId, &Transform, &Sprite)>,
    mut hitboxes: ResMut<Hitboxes>,
) {
    for (unit_id, transform, sprite) in movables {
        hitboxes.values.insert(unit_id.clone(),(transform.translation.xy(), sprite.custom_size.expect("BUG - Sprite must have a custom size")));
    }
}

pub fn move_units(
    movables: Query<(&UnitId, &mut Moving, &mut Transform, &Sprite), With<Pickable>>,
    new_positions: ResMut<NewPositions>,
    time: Res<Time>,
    hitboxes: Res<Hitboxes>,
) {
    let delta = time.delta_secs();
    for (unit_id, mut moving, mut transform, sprite) in movables {
        let Some(target_position) = new_positions.get(unit_id) else {
            continue;
        };
        let mut current_position = transform.translation.xy();
        if current_position.x == target_position.x && current_position.y == target_position.y {
            *moving = Moving(false);
            continue;
        }
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
        let mut is_hit = false;
        for (other_id, (other_position, other_size)) in &hitboxes.values {
            if unit_id == other_id {continue;}
            if is_entity_in_hitbox(current_position, *other_position, *other_size) {
                is_hit = true;
            }
        }
        if is_hit {continue;}
        transform.translation.x = current_position.x;
        transform.translation.y = current_position.y;
    }
}
