use bevy::prelude::*;

use crate::ingame::resources::dungeon::wave::Wave;
use crate::scenes::SceneState;

pub fn countdown(time: Res<Time>, mut wave: ResMut<Wave>, mut state: ResMut<State<SceneState>>) {
    wave.timer.tick(time.delta());
    if wave.timer.finished() {
        state.push(SceneState::RewardsScene).unwrap();
    }
}
