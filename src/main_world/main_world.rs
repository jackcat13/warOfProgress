use bevy::prelude::*;

use super::{hud::setup_hud, villagers::setup_villagers};

pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let villager_asset = asset_server.load("caveman_age/units/units_caveman.png");
    setup_villagers(
        &mut commands,
        villager_asset,
        meshes.add(Annulus::new(17., 20.)),
        materials.add(Color::srgb(0., 0.5, 0.)),
    );
    let house_asset = asset_server.load("caveman_age/buildings/house_caveman.png");
    setup_hud(&mut commands, house_asset);
}
