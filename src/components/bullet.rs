use bevy::prelude::*;

#[derive(Component)]
pub struct BulletComponent {
    pub duration: Timer,
    pub target_x: f32,
    pub target_y: f32,
    pub speed: f32,
}
