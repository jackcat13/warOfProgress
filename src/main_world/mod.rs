use bevy::prelude::*;
use std::fmt::Debug;

#[derive(Component)]
pub struct Villager;

pub fn setup_villagers(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2 { x: 50., y: 70. }),
            image: asset_server.load("caveman_age/units/units_caveman.png"),
            ..default()
        },
        Villager,
        Pickable::default(),
    ))
        .observe(recollor::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(recollor::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)));
}

fn recollor<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        sprite.color = color;
    }
}


