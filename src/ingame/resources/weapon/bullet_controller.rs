use serde::{Deserialize, Serialize};

use crate::ingame::resources::weapon::bullet::Bullet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulletController {
    pub bullet_information: Bullet,
    pub spawn_bullet: bool,
    pub target_x: f32,
    pub target_y: f32,
}
