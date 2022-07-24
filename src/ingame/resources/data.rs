use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::DATA_FILE;
use crate::ingame::resources::effect::effect_information::EffectInformation;
use crate::ingame::resources::hero::hero_class::HeroClass;
use crate::ingame::resources::hero::Hero;
use crate::ingame::resources::skill::Skill;
use crate::ingame::resources::weapon::Weapon;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    heros: [Hero; 4],
    weapons: [Weapon; 11],
    skills: [Skill; 4],
    player_effect_information: [EffectInformation; 6],
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

    pub fn get_skill(&self, hero_class: HeroClass) -> Skill {
        let hero = self.get_hero(hero_class);
        self.skills
            .iter()
            .find(|skill| skill.name == hero.skill)
            .unwrap()
            .clone()
    }

    pub fn get_hero(&self, hero_class: HeroClass) -> Hero {
        self.heros
            .iter()
            .find(|hero| hero.hero_class == hero_class)
            .unwrap()
            .clone()
    }

    pub fn get_weapon(&self, hero_class: HeroClass) -> Weapon {
        let hero = self.get_hero(hero_class);
        self.weapons
            .iter()
            .find(|weapon| weapon.name == hero.weapon)
            .unwrap()
            .clone()
    }

    pub fn get_player_effect_information(&self) -> Vec<EffectInformation> {
        self.player_effect_information.to_vec().clone()
    }
}
