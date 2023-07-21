use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, InspectorOptions)]
pub enum HeroClass {
    Elf,
    Knight,
    Wizard,
    Lizard,
}

impl HeroClass {
    pub fn iterator() -> Iter<'static, HeroClass> {
        [
            HeroClass::Elf,
            HeroClass::Knight,
            HeroClass::Wizard,
            HeroClass::Lizard,
        ]
        .iter()
    }
}
