use bevy::{
    color::palettes::tailwind::{SKY_700, SLATE_50},
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{camera::MainCamera, mouse::get_mouse_position};

use super::{
    observers::{menu_action, recollor},
    world_components::{CurrentMouseAsset, MenuAction, MouseComponent},
};

pub fn setup_hud(commands: &mut Commands, house_asset: Handle<Image>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.),
            row_gap: Val::Px(10.),
            ..default()
        },
        Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            parent
                .spawn(button("House", house_asset))
                .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
                .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
                .observe(menu_action::<Pointer<Released>>(MenuAction::House));
        }),)),
    ));
    commands.spawn((
        Sprite {
            image: default(),
            ..default()
        },
        Transform::default(),
        MouseComponent,
    ));
}

fn button<T: Into<String>>(text: T, button_asset: Handle<Image>) -> impl Bundle {
    (
        Button,
        BackgroundColor(SKY_700.into()),
        Node {
            padding: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            width: Val::Px(200.),
            height: Val::Px(100.),
            ..default()
        },
        children![
            (Text::new(text), TextColor(SLATE_50.into())),
            (
                ImageNode {
                    image: button_asset,
                    ..default()
                },
                Transform::from_scale(Vec3 {
                    x: 0.6,
                    y: 0.6,
                    z: 0.
                })
            )
        ],
    )
}

pub fn draw_mouse_asset(
    current_mouse_asset: Res<CurrentMouseAsset>,
    mut sprite: Query<&mut Sprite, With<MouseComponent>>,
    mut transform: Query<&mut Transform, With<MouseComponent>>,
    window: Single<&Window>,
    camera: Single<&Camera, With<MainCamera>>,
    camera_transform: Single<&GlobalTransform, With<MainCamera>>,
) {
    // The 2 following lines are needed because for some reason, Single does not work for these two
    // -> Query usage instead
    let Some(mut sprite) = sprite.iter_mut().next() else {
        return;
    };
    let Some(mut transform) = transform.iter_mut().next() else {
        return;
    };
    let Some(mouse_position) = get_mouse_position(window, camera, camera_transform) else {
        return;
    };
    let Some(asset) = &current_mouse_asset.asset else {
        sprite.image = default();
        sprite.custom_size = None;
        return;
    };
    sprite.image = asset.clone();
    sprite.custom_size = Some(Vec2 { x: 60., y: 60. });
    transform.translation.x = mouse_position.x;
    transform.translation.y = mouse_position.y;
}
