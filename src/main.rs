use bevy::prelude::*;
use camera::setup_camera;
use main_world::setup_villagers;

mod camera;
mod main_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_villagers))
        .run();
}

