use bevy::{input::InputSystem, prelude::*};
use std::fmt::Debug;
use uuid::Uuid;

use crate::camera::MainCamera;

#[derive(Component, PartialEq)]
pub struct Villager;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Selected {
    entities: Vec<UnitId>,
}

#[derive(Component, PartialEq, Clone, Deref, DerefMut)]
pub struct UnitId(String);

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
                    custom_size: Some(Vec2 { x: 50., y: 70. }),
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
                    UnitId(uuid),
                    Mesh2d(meshes.add(Annulus::new(0., 0.))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 0.5, 0.))),
                    Transform::from_xyz(-5., -30., 0.).with_rotation(Quat::from_rotation_x(90.)),
                ));
            })
            .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
            .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
            .observe(focus::<Pointer<Pressed>>());
    }
}

pub fn highlight_selected_units(
    query: Query<(&UnitId, &mut Mesh2d), With<UnitId>>,
    selected: Res<Selected>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (id, mut mesh) in query {
        *mesh = if selected.contains(id) {
            Mesh2d(meshes.add(Annulus::new(17., 20.)))
        } else {
            Mesh2d(meshes.add(Annulus::new(0., 0.)))
        }
    }
}

pub fn check_movement_on_right_click(
    query: Query<(&UnitId, &Transform), With<Pickable>>,
    selected: Res<Selected>,
    window: Single<&Window>,
    camera_single: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if !buttons.just_released(MouseButton::Right) { return; }
    let (camera, camera_transform) = (camera_single.0, camera_single.1);
    for (unit_id, transorm) in query {
        if !selected.contains(unit_id) { continue; }
        let Some(Ok(world_position)) = window
            .cursor_position()
            .map(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        else {
            continue;
        };
        println!("{:?}", world_position);
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
