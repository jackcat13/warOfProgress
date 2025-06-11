use bevy::prelude::*;

#[derive(Component)]
pub struct BasicCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
