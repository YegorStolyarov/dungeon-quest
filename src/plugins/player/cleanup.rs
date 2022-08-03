use bevy::prelude::*;

use crate::{
    plugins::player::PlayerEntity,
    resources::{dungeon::wave::Wave, profile::Profile},
};

pub fn cleanup_player(mut commands: Commands, player_entity: Res<PlayerEntity>) {
    commands.entity(player_entity.entity).despawn_recursive();
}

pub fn save_cleared_waves(wave: Res<Wave>, mut profile: ResMut<Profile>) {
    profile.total_cleared_waves = wave.wave_number - 1;
}
