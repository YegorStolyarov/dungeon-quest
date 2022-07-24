use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum WeaponType {
    BowAndArrow,
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
