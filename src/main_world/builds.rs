use bevy::prelude::*;
use uuid::Uuid;

use crate::{camera::MainCamera, mouse::get_mouse_position};

use super::world_components::{
    BuildingCost, CurrentMouseAsset, House, MouseComponent, PlayerResources, UnitId,
};

pub fn build_check(
    buttons: Res<ButtonInput<MouseButton>>,
    mut current_mouse_asset: ResMut<CurrentMouseAsset>,
    window: Single<&Window>,
    camera: Single<&Camera, With<MainCamera>>,
    camera_transform: Single<&GlobalTransform, With<MainCamera>>,
    mut player_resources: ResMut<PlayerResources>,
    mut sprite: Query<&mut Sprite, With<MouseComponent>>,
    mut commands: Commands,
) {
    let Some(mut sprite) = sprite.iter_mut().next() else {
        return;
    };
    sprite.color = Color::srgb(1., 0., 0.);
    let Some(asset) = &mut current_mouse_asset.asset else {
        return;
    };
    let building_cost = resolve_building_from_asset(asset);
    let Some(mouse_position) = get_mouse_position(window, camera, camera_transform) else {
        return;
    };
    if !can_pay(&building_cost, &player_resources) {
        return;
    }
    sprite.color = Color::srgb(0., 1., 0.);
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    };
    player_resources.wood -= building_cost.wood_cost;
    player_resources.stone -= building_cost.stone_cost;
    player_resources.gold -= building_cost.gold_cost;
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2 { x: 60., y: 60. }),
            image: sprite.image.clone(),
            ..default()
        },
        Transform::from_xyz(mouse_position.x, mouse_position.y, 0.),
        House,
        UnitId(Uuid::new_v4().to_string()),
    ));
    sprite.image = default();
    current_mouse_asset.asset = None;
}

fn can_pay(building_cost: &BuildingCost, player_resources: &PlayerResources) -> bool {
    if player_resources.wood < building_cost.wood_cost {
        return false;
    };
    if player_resources.stone < building_cost.stone_cost {
        return false;
    };
    if player_resources.gold < building_cost.gold_cost {
        return false;
    };
    true
}

fn resolve_building_from_asset(asset: &mut Handle<Image>) -> BuildingCost {
    let path = asset
        .path()
        .expect("BUG - Asset must have a path")
        .to_string();
    if path.contains("house") {
        BuildingCost {
            wood_cost: 50,
            stone_cost: 0,
            gold_cost: 0,
        }
    } else {
        panic!("BUG - The asset must have a Building associated : {}", path);
    }
}
