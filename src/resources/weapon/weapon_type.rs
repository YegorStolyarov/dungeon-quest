use bevy_inspector_egui::prelude::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, InspectorOptions)]
pub enum WeaponType {
    Bow,
    Spear,
    ShortSword,
    Sword,
    BigMachete,
    SmallWand,
    MagicWand,
    MagicSword,
    SmallHammer,
    Mace,
    BigHammer,
}
