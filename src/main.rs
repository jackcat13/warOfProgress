use bevy::prelude::*;
use camera::setup_camera;
use main_world::{setup_villagers, Selected};

mod camera;
mod main_world;

fn main() {
    App::new()
        .init_resource::<Selected>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_villagers))
        .run();
}

