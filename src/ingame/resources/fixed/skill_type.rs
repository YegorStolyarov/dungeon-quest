use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SkillType {
    TimeToHunt,
    Armor,
    Thunderstorm,
    AnimalInstinct,
}
