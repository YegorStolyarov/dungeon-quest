use bevy::prelude::*;

use crate::plugins::gameplay::camera::*;
use crate::plugins::gameplay::player::*;
use crate::plugins::gameplay::shared_systems::*;

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
                .with_system(player_animation_system)
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
