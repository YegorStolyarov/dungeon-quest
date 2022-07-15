use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::DATA_FILE;

use crate::ingame::resources::fixed::effect::Effect;
use crate::ingame::resources::fixed::hero::Hero;
use crate::ingame::resources::fixed::skill::Skill;
use crate::ingame::resources::fixed::weapon::Weapon;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub heros: [Hero; 4],
    pub weapons: [Weapon; 11],
    pub effects: [Effect; 10],
    pub skills: [Skill; 4],
}

impl Data {
    pub fn new() -> Self {
        let data: Data = match File::open(DATA_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can't find language file: {}", err.to_string()),
        };
        data
    }
}
