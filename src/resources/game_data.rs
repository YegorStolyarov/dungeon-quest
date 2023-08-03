use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use crate::config::DATA_FILE;
use crate::resources::effect::Effect;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::hero::Hero;
use crate::resources::monster::Monster;
use crate::resources::skill::Skill;
use crate::resources::weapon::weapon_type::WeaponType;
use crate::resources::weapon::Weapon;

#[derive(Resource)]
pub struct PauseSceneData {
    pub(crate) user_interface_root: Entity,
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    heroes: [Hero; 4],
    weapons: [Weapon; 11],
    skills: [Skill; 4],
    player_list_effects_information: [Effect; 8],
    monsters: [Monster; 10],
}

impl GameData {
    pub fn new() -> Self {
        let data = match File::open(DATA_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can't find language file: {}", err),
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
        self.heroes
            .iter()
            .find(|hero| hero.hero_class == hero_class)
            .unwrap()
            .clone()
    }

    pub fn get_weapon(&self, hero_class: HeroClass) -> Weapon {
        let hero = self.get_hero(hero_class);
        *self
            .weapons
            .iter()
            .find(|weapon| weapon.name == hero.weapon)
            .unwrap()
    }

    pub fn get_weapons(&self, hero_class: HeroClass) -> Vec<Weapon> {
        let weapons_type: Vec<WeaponType> = match hero_class {
            HeroClass::Elf => [WeaponType::Bow, WeaponType::Spear].to_vec(),
            HeroClass::Knight => [
                WeaponType::ShortSword,
                WeaponType::Sword,
                WeaponType::BigMachete,
            ]
            .to_vec(),
            HeroClass::Wizard => [
                WeaponType::SmallWand,
                WeaponType::MagicWand,
                WeaponType::MagicSword,
            ]
            .to_vec(),
            HeroClass::Lizard => [
                WeaponType::SmallHammer,
                WeaponType::Mace,
                WeaponType::BigHammer,
            ]
            .to_vec(),
        };

        self.weapons
            .iter()
            .filter(|weapon| weapons_type.contains(&weapon.name))
            .cloned()
            .collect()
    }

    pub fn get_player_list_effects_information(&self) -> Vec<Effect> {
        self.player_list_effects_information.to_vec()
    }

    pub fn get_monsters(&self) -> Vec<Monster> {
        self.monsters.to_vec()
    }
}
