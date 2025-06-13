use bevy::prelude::*;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Component, PartialEq)]
pub struct Villager;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Selected {
    entities: Vec<UnitId>,
}

#[derive(Component, PartialEq, Clone, Deref, DerefMut)]
pub struct UnitId(String);

pub fn setup_villagers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let villager_asset = asset_server.load("caveman_age/units/units_caveman.png");
    for index in 0..2 {
        commands
            .spawn((
                Sprite {
                    custom_size: Some(Vec2 { x: 50., y: 70. }),
                    image: villager_asset.clone(),
                    ..default()
                },
                Transform::from_xyz(index as f32 * 100., 0., 0.),
                Villager,
                UnitId(Uuid::new_v4().to_string()),
                Pickable::default(),
            ))
            .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
            .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
            .observe(focus::<Pointer<Pressed>>());
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

fn focus<E>() -> impl Fn(Trigger<E>, Query<(&Sprite, &UnitId)>, ResMut<Selected>)
where
    E: Debug + Clone + Reflect,
{
    move |ev, query, mut selected| {
        let Ok((sprite, uuid)) = query.get(ev.target()) else {
            return;
        };
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
