use bevy::prelude::*;

use crate::ingame::player::PlayerEntity;

pub fn cleanup_player(mut commands: Commands, player_entity: Res<PlayerEntity>) {
    commands.entity(player_entity.entity).despawn_recursive();
}
