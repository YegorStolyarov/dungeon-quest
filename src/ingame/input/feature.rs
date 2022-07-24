use bevy::prelude::*;

use crate::scenes::SceneState;

pub fn pause(mut keyboard_input: ResMut<Input<KeyCode>>, mut state: ResMut<State<SceneState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state.push(SceneState::PauseScene).unwrap();
        keyboard_input.reset(KeyCode::Escape);
    }
}
