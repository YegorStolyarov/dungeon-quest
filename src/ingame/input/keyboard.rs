use bevy::prelude::*;

use crate::scenes::SceneState;

pub fn escape(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<SceneState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state
            .set(SceneState::ResultScene)
            .expect("Couldn't switch state to Result Scene");
    }
}
