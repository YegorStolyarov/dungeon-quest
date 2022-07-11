use serde::{Deserialize, Serialize};

pub struct Hero {
    gender: Gender,
    class: HeroClass,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HeroClass {
    Elf,
    Knight,
    Wizard,
    Lizard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}
