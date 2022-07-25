use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UpgradeType {
    Weapon,
    Effect,
    Stats,
    Skill,
}
