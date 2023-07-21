use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, InspectorOptions)]
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
