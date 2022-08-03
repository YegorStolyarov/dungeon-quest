use bevy::prelude::*;

use crate::components::{
    invinsible_cooldown::InvisibleCooldownComponent, monster::MonsterComponent,
};

pub fn hurt_duration_color(
    mut invinsible_cooldown_query: Query<
        (&mut InvisibleCooldownComponent, &mut TextureAtlasSprite),
        With<MonsterComponent>,
    >,
    time: Res<Time>,
) {
    for (mut invinsible_cooldown, mut texture) in invinsible_cooldown_query.iter_mut() {
        if !invinsible_cooldown.hurt_duration.finished() {
            texture.color = Color::RED;
            invinsible_cooldown.hurt_duration.tick(time.delta());
        } else {
            texture.color = Color::default();
        }
    }
}
