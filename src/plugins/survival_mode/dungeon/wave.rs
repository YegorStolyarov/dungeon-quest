use bevy::prelude::*;

use crate::resources::dungeon::wave::Wave;
use crate::scenes::SceneState;

pub fn countdown(time: Res<Time>, mut wave: ResMut<Wave>, mut state: ResMut<NextState<SceneState>>) {
    wave.timer.tick(time.delta());
    if wave.timer.finished() {
        state.set(SceneState::RewardsScene);
    }
}
