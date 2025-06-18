use bevy::prelude::*;
use camera::setup_camera;
use main_world::{
    check_movement_on_right_click, draw_mouse_asset, highlight_selected_units, move_units,
    setup_villagers, CurrentMouseAsset, NewPositions, Selected,
};

mod camera;
mod main_world;

fn main() {
    App::new()
        .init_resource::<Selected>()
        .init_resource::<NewPositions>()
        .init_resource::<CurrentMouseAsset>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_villagers))
        .add_systems(
            Update,
            (
                highlight_selected_units,
                check_movement_on_right_click,
                move_units,
                draw_mouse_asset,
            ),
        )
        .run();
}
