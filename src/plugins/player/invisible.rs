use bevy::{
    prelude::{Color, Query, Res, Time, With},
    sprite::TextureAtlasSprite,
};

use crate::components::{invinsible_cooldown::InvisibleCooldownComponent, player::PlayerComponent};

pub fn invinsible_cooldown(
    mut invinsible_cooldown_query: Query<&mut InvisibleCooldownComponent, With<PlayerComponent>>,
    time: Res<Time>,
) {
    let mut invinsible_cooldown = invinsible_cooldown_query.single_mut();

    if !invinsible_cooldown.duration.finished() {
        invinsible_cooldown.duration.tick(time.delta());
    }

    if !invinsible_cooldown.hurt_duration.finished() {
        invinsible_cooldown.hurt_duration.tick(time.delta());
    }
}

pub fn hurt_duration_color(
    mut invinsible_cooldown_query: Query<
        (&InvisibleCooldownComponent, &mut TextureAtlasSprite),
        With<PlayerComponent>,
    >,
) {
    let (invinsible_cooldown, mut texture) = invinsible_cooldown_query.single_mut();

    if !invinsible_cooldown.hurt_duration.finished() {
        texture.color = Color::RED;
    } else {
        texture.color = Color::default();
    }
}
