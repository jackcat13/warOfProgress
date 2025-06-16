use bevy::prelude::*;
use camera::setup_camera;
use main_world::{check_movement_on_right_click, highlight_selected_units, move_units, setup_villagers, NewPositions, Selected};

mod camera;
mod main_world;

fn main() {
    App::new()
        .init_resource::<Selected>()
        .init_resource::<NewPositions>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_villagers))
        .add_systems(Update, highlight_selected_units)
        .add_systems(Update, check_movement_on_right_click)
        .add_systems(Update, move_units)
        .run();
}

