use bevy::prelude::*;
use camera::setup_camera;
use main_world::{
    builds::build_check,
    hud::{draw_mouse_asset, update_resources},
    main_world::setup_world,
    units::{check_build_target_on_right_click, check_movement_on_right_click, highlight_selected_units, move_units, process_building_builds, process_hitboxes},
    world_components::{CurrentMouseAsset, Hitboxes, NewPositions, PlayerResources, Selected},
};

mod camera;
mod main_world;
mod mouse;

fn main() {
    App::new()
        .init_resource::<Selected>()
        .init_resource::<NewPositions>()
        .init_resource::<CurrentMouseAsset>()
        .init_resource::<PlayerResources>()
        .init_resource::<Hitboxes>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_world))
        .add_systems(
            Update,
            (
                highlight_selected_units,
                check_movement_on_right_click,
                check_build_target_on_right_click,
                move_units,
                process_building_builds,
                draw_mouse_asset,
                update_resources,
                build_check,
                process_hitboxes,
            ),
        )
        .run();
}
