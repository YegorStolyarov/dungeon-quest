use bevy::prelude::*;

#[derive(Component)]
pub struct InvisibleCooldownComponent {
    pub hurt_duration: Timer,
    pub duration: Timer,
}
