use bevy::prelude::*;

use crate::components::{
    invinsible_cooldown::InvisibleCooldownComponent, monster::MonsterComponent,
};

pub fn hurt_duration_color(
    mut invincible_cooldown_query: Query<
        (&mut InvisibleCooldownComponent, &mut TextureAtlasSprite),
        With<MonsterComponent>,
    >,
    time: Res<Time>,
) {
    for (mut invincible_cooldown, mut texture) in invincible_cooldown_query.iter_mut() {
        if !invincible_cooldown.hurt_duration.finished() {
            texture.color = Color::RED;
            invincible_cooldown.hurt_duration.tick(time.delta());
        } else {
            texture.color = Color::default();
        }
    }
}
