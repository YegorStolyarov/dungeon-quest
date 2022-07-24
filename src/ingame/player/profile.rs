use bevy::prelude::*;
use chrono::{DateTime, Local};

use crate::ingame::resources::profile::Profile;
use crate::scenes::SceneState;

pub fn finish_run(mut profile: ResMut<Profile>, mut state: ResMut<State<SceneState>>) {
    if profile.is_run_finished {
        let end_time: DateTime<Local> = Local::now();
        profile.end_time = end_time.to_rfc3339();

        state
            .set(SceneState::ResultScene)
            .expect("Can't change to result scene");
    }
}
