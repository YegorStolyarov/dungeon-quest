use bevy::prelude::*;

use crate::plugins::weapon::WeaponEntity;
use crate::components::bullet::BulletComponent;

pub fn cleanup_weapon(mut commands: Commands, weapon_entity: Res<WeaponEntity>) {
    commands.entity(weapon_entity.entity).despawn_recursive();
}

pub fn cleanup_bullet(mut commands: Commands, bullets_query: Query<Entity, With<BulletComponent>>) {
    for entity in bullets_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}