use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn iterator() -> Iter<'static, Gender> {
        [Gender::Male, Gender::Female].iter()
    }
}
