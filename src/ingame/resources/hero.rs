use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HeroClass {
    Elf,
    Knight,
    Wizard,
    Lizard,
}

impl HeroClass {
    pub fn iterator() -> Iter<'static, HeroClass> {
        static HERO_CLASSES: [HeroClass; 4] = [
            HeroClass::Elf,
            HeroClass::Knight,
            HeroClass::Wizard,
            HeroClass::Lizard,
        ];
        HERO_CLASSES.iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn iterator() -> Iter<'static, Gender> {
        static GENDERS: [Gender; 2] = [Gender::Male, Gender::Female];
        GENDERS.iter()
    }
}
