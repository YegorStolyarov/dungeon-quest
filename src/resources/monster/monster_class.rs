use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable)]
pub enum MonsterClass {
    SmallZombie,
    Zombie,
    BigZombie,
    Goblin,
    Orc,
    Ogre,
    Imp,
    Chort,
    BigDemon,
    Swampy,
}
