use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone, InspectorOptions)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn iterator() -> Iter<'static, Gender> {
        [Gender::Male, Gender::Female].iter()
    }
}
