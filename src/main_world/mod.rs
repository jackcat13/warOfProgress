use bevy::prelude::*;

pub fn setup_villagers(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        Sprite {
            custom_size: Some(Vec2 { x: 50., y: 70. }),
            image: asset_server.load("caveman_age/units/units_caveman.png"),
            ..default()
        },
    );
}
