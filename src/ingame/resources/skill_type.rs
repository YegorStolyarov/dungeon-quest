use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SkillType {
    TimeToHunt,
    Armor,
    Thunderstorm,
    AnimalInstinct,
}
