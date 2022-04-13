use bevy::prelude::*;

use crate::plugins::gameplay::camera::*;
use crate::plugins::gameplay::player::*;
use crate::state::*;

pub struct MovementDemoPlugin;

impl Plugin for MovementDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ApplicationState::MovementDemo)
                .with_system(setup_camera)
                .with_system(setup_player),
        );
        app.add_system_set(
            SystemSet::on_update(ApplicationState::MovementDemo)
                .with_system(player_movement_system)
                .with_system(escape_button_handle),
        );
        app.add_system_set(
            SystemSet::on_exit(ApplicationState::MovementDemo)
                .with_system(cleanup_camera)
                .with_system(cleanup_player),
        );
    }
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
