use bevy::prelude::*;

use crate::config::*;

pub struct Camera {
    entity: Entity,
}

pub fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    let camera_entity = commands.spawn_bundle(camera).id();

    commands.insert_resource(Camera {
        entity: camera_entity,
    });
}

pub fn cleanup_camera(mut commands: Commands, camera: Res<Camera>) {
    commands.entity(camera.entity).despawn_recursive();
}
