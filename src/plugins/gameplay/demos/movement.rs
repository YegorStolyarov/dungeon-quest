use bevy::prelude::*;

use crate::config::*;
use crate::plugins::gameplay::player::*;
use crate::state::*;

struct MovementDemoData {
    camera_entity: Entity,
}

pub struct MovementDemoPlugin;

impl Plugin for MovementDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ApplicationState::MovementDemo)
                .with_system(setup)
                .with_system(setup_player),
        );
        app.add_system_set(
            SystemSet::on_update(ApplicationState::MovementDemo)
                .with_system(player_movement_system)
                .with_system(escape_button_handle),
        );
        app.add_system_set(
            SystemSet::on_exit(ApplicationState::MovementDemo)
                .with_system(cleanup)
                .with_system(clean_up_player),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Movement Demo");
    print!("HEY");

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    let camera_entity = commands.spawn_bundle(camera).id();

    commands.insert_resource(MovementDemoData { camera_entity });
}

fn cleanup(mut commands: Commands, menu_data: Res<MovementDemoData>) {
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

fn escape_button_handle(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<ApplicationState>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state
            .set(ApplicationState::DemosMenu)
            .expect("Couldn't switch state to DemoMenu");
    }
}
