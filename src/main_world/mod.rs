use bevy::{
    color::palettes::tailwind::{SKY_700, SLATE_50},
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    platform::collections::HashMap,
    prelude::*,
};
use std::fmt::Debug;
use uuid::Uuid;

use crate::camera::MainCamera;

const SPEED: f32 = 100.0;

#[derive(Component, PartialEq)]
pub struct Villager;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Selected {
    entities: Vec<UnitId>,
}

#[derive(Component)]
pub struct SelectChild;

#[derive(Component, Eq, Hash, PartialEq, Clone, Deref, DerefMut)]
pub struct UnitId(String);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct NewPositions {
    positions: HashMap<UnitId, Vec2>,
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct CurrentMouseAsset {
    asset: Option<Handle<Image>>,
}

#[derive(Component)]
pub struct MouseComponent;

#[derive(Debug)]
enum MenuAction {
    House,
}

pub fn setup_villagers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let villager_asset = asset_server.load("caveman_age/units/units_caveman.png");
    for index in 0..2 {
        let uuid = Uuid::new_v4().to_string();
        commands
            .spawn((
                Sprite {
                    custom_size: Some(Vec2 { x: 35., y: 45. }),
                    image: villager_asset.clone(),
                    ..default()
                },
                Transform::from_xyz(index as f32 * 100., 0., 0.),
                Villager,
                UnitId(uuid.clone()),
                Pickable::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SelectChild,
                    UnitId(uuid.clone()),
                    Mesh2d(meshes.add(Annulus::new(17., 20.))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 0.5, 0.))),
                    Transform::from_xyz(-5., -17., 0.).with_rotation(Quat::from_rotation_x(90.)),
                    Visibility::Hidden,
                ));
            })
            .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
            .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
            .observe(focus::<Pointer<Pressed>>());
        commands.spawn((
            SelectChild,
            UnitId(uuid),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.),
                row_gap: Val::Px(10.),
                ..default()
            },
            Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                parent
                    .spawn(button("House"))
                    .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
                    .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
                    .observe(menu_action::<Pointer<Pressed>>(MenuAction::House));
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
}

fn menu_action<E: Debug + Clone + Reflect>(
    menu_action: MenuAction,
) -> impl Fn(Trigger<E>, Res<AssetServer>, ResMut<CurrentMouseAsset>) {
    move |_ev, asset_server, mut current_mouse_asset| {
        current_mouse_asset.asset = match menu_action {
            MenuAction::House => Some(asset_server.load("caveman_age/buildings/house_caveman.png")),
        };
    }
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
    let Some(mouse_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(&camera_transform, cursor).ok()) else {
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

fn button<T: Into<String>>(text: T) -> impl Bundle {
    (
        Button,
        BackgroundColor(SKY_700.into()),
        Node {
            padding: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            width: Val::Px(200.),
            ..default()
        },
        children![(Text::new(text), TextColor(SLATE_50.into()))],
    )
}

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
    mut new_positions: ResMut<NewPositions>,
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

fn recollor<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        sprite.color = color;
    }
}

fn focus<E>() -> impl Fn(Trigger<E>, Query<(&mut Sprite, &UnitId)>, ResMut<Selected>)
where
    E: Debug + Clone + Reflect,
{
    move |ev, mut query, mut selected| {
        let Ok((_sprite, uuid)) = query.get_mut(ev.target()) else {
            return;
        };
        if selected.entities.contains(uuid) {
            return;
        }
        selected.entities.clear();
        selected.entities.push(uuid.clone());
        println!(
            "Selected entities : {}",
            selected
                .entities
                .iter()
                .map(|it| it.to_string())
                .collect::<String>()
        );
    }
}
