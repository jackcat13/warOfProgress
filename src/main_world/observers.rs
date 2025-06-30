use bevy::{ecs::component::ComponentIdFor, prelude::*};

use std::fmt::Debug;

use crate::main_world::world_components::{Selected, UnitId};

use super::world_components::{BuildTarget, CurrentMouseAsset, MenuAction, Villager};

pub fn recollor<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        sprite.color = color;
    }
}

pub fn focus<E>() -> impl Fn(Trigger<E>, Query<(&mut Sprite, &UnitId)>, ResMut<Selected>)
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

pub fn menu_action<E: Debug + Clone + Reflect>(
    menu_action: MenuAction,
) -> impl Fn(Trigger<E>, Res<AssetServer>, ResMut<CurrentMouseAsset>) {
    move |_ev, asset_server, mut current_mouse_asset| {
        current_mouse_asset.asset = match menu_action {
            MenuAction::House => Some(asset_server.load("caveman_age/buildings/house_caveman.png")),
        };
    }
}
