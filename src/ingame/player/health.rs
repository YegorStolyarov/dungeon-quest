use bevy::prelude::*;

use crate::ingame::resources::player::Player;
use crate::ingame::resources::profile::Profile;

pub fn end_run_check(mut player_query: Query<&mut Player>, mut profile: ResMut<Profile>) {
    let player = player_query.single_mut();
    if player.current_health_points == 0.0 {
        profile.is_run_finished = true;
    }
}
