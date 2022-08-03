use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable)]
pub enum MonsterClass {
    SmallZombie,
    Zombie,
    BigZombie,
    Goblin,
    Orc,
    Orge,
    Imp,
    Chort,
    BigDemon,
    Swampy,
}
