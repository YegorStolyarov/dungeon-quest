use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PowerBonus {
    Strength,
    Intelligence,
}
