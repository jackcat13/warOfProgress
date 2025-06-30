use bevy::{prelude::*, sprite::Material2d};
use uuid::Uuid;

use super::{
    observers::{focus, recollor},
    world_components::{BuildTarget, SelectChild, UnitId, Villager},
};

pub fn setup_villagers<M: Material2d>(
    commands: &mut Commands,
    villager_asset: Handle<Image>,
    select_mesh: Handle<Mesh>,
    select_mesh_material: Handle<M>,
) {
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
                BuildTarget { id: None },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SelectChild,
                    UnitId(uuid),
                    Mesh2d(select_mesh.clone()),
                    MeshMaterial2d(select_mesh_material.clone()),
                    Transform::from_xyz(-5., -17., 0.).with_rotation(Quat::from_rotation_x(90.)),
                    Visibility::Hidden,
                ));
            })
            .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
            .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
            .observe(focus::<Pointer<Pressed>>());
    }
}
