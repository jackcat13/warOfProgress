use bevy::prelude::*;
use uuid::Uuid;

use crate::{camera::MainCamera, mouse::get_mouse_position};

use super::world_components::{
    Achievement, BuildingCost, BuildingSpecs, CurrentMouseAsset, House, MenuAction, MouseComponent, PlayerResources, UnitId
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
    mut asset_server: ResMut<AssetServer>,
) {
    let Some(mut sprite) = sprite.iter_mut().next() else {
        return;
    };
    sprite.color = Color::srgb(1., 0., 0.);
    let Some(asset) = &mut current_mouse_asset.asset else {
        return;
    };
    let building_specs = resolve_building_from_asset(asset);
    let Some(mouse_position) = get_mouse_position(window, camera, camera_transform) else {
        return;
    };
    if !can_pay(&building_specs, &player_resources) {
        return;
    }
    sprite.color = Color::srgb(0., 1., 0.);
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    };
    player_resources.wood -= building_specs.cost.wood;
    player_resources.stone -= building_specs.cost.stone;
    player_resources.gold -= building_specs.cost.gold;
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2 { x: 60., y: 60. }),
            image: asset_server.load(building_specs.resolve_in_progress_asset_path()),
            ..default()
        },
        Transform::from_xyz(mouse_position.x, mouse_position.y, 0.),
        House,
        Achievement { progress: 0.0 },
        UnitId(Uuid::new_v4().to_string()),
    ));
    sprite.image = default();
    current_mouse_asset.asset = None;
}

fn can_pay(building_specs: &BuildingSpecs, player_resources: &PlayerResources) -> bool {
    let building_cost = &building_specs.cost;
    if player_resources.wood < building_cost.wood {
        return false;
    };
    if player_resources.stone < building_cost.stone {
        return false;
    };
    if player_resources.gold < building_cost.gold {
        return false;
    };
    true
}

fn resolve_building_from_asset(asset: &mut Handle<Image>) -> BuildingSpecs {
    let path = asset
        .path()
        .expect("BUG - Asset must have a path")
        .to_string();
    if path.contains("house") {
        BuildingSpecs {
            r#type: MenuAction::House,
            cost: BuildingCost {
                wood: 50,
                stone: 0,
                gold: 0,
            }
        }
    } else {
        panic!("BUG - The asset must have a Building associated : {}", path);
    }
}
