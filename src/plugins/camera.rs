use bevy::prelude::*;

#[derive(Component)]
pub struct UserInterfaceCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_user_interface_camera);
    }
}

fn spawn_user_interface_camera(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UserInterfaceCamera);
}
