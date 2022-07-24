use serde::{Deserialize, Serialize};
use std::slice::Iter;

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
