use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Inspectable, Copy)]
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
