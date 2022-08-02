use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::config::*;
use crate::scenes::SceneState;

#[derive(Component)]
pub struct UserInterfaceCamera;

#[derive(Component)]
pub struct Orthographic2DCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_user_interface_camera);
        app.add_startup_system(spawn_2d_camera);

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode).with_system(camera_follow),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameSurvivalMode).with_system(reset_camera),
        );
    }
}

fn spawn_user_interface_camera(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Name::new("UserInterfaceCamera"))
        .insert(UserInterfaceCamera);
}

fn spawn_2d_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    commands
        .spawn_bundle(camera)
        .insert(Orthographic2DCamera)
        .insert(Name::new("Orthographic2DCamera"));
}

fn camera_follow(
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut camera_query: Query<&mut Transform, (Without<PlayerComponent>, With<Orthographic2DCamera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn reset_camera(mut camera_query: Query<&mut Transform, With<Orthographic2DCamera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}
