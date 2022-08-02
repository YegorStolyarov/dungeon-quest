use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::skill::skill_type::SkillType;
use crate::resources::upgrade::upgrade_type::UpgradeType;
use crate::resources::upgrade::Upgrade;

pub struct UpgradeController {
    pub upgrades: Vec<Upgrade>,
}

impl UpgradeController {
    pub fn new() -> Self {
        let upgrades: Vec<Upgrade> = match File::open(UPGRADES_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can't find language file: {}", err),
        };

        UpgradeController { upgrades }
    }

    pub fn get_skill_upgrade(&self, skill_name: SkillType) -> Upgrade {
        let skill_upgrades: Vec<Upgrade> = self
            .upgrades
            .iter()
            .filter(|upgrade| {
                upgrade.upgrade_type == UpgradeType::Skill
                    && upgrade.skill_upgrade.clone().unwrap().name == skill_name
            })
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..skill_upgrades.len());
        skill_upgrades[random_index].clone()
    }

    pub fn get_stats_upgrade(&self) -> Upgrade {
        let stats_upgrades: Vec<Upgrade> = self
            .upgrades
            .iter()
            .filter(|upgrade| upgrade.upgrade_type == UpgradeType::Stats)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..stats_upgrades.len());
        stats_upgrades[random_index].clone()
    }

    pub fn get_effect_upgrade(&self) -> Upgrade {
        let effect_upgrades: Vec<Upgrade> = self
            .upgrades
            .iter()
            .filter(|upgrade| upgrade.upgrade_type == UpgradeType::Effect)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..effect_upgrades.len());
        effect_upgrades[random_index].clone()
    }

    pub fn get_three_upgrades(&self, hero_class: HeroClass, weapon_level: u8) -> Vec<UpgradeType> {
        let mut three_upgrades: Vec<UpgradeType> = Vec::new();
        let mut upgrade_types: Vec<UpgradeType> = vec![UpgradeType::Stats, UpgradeType::Skill, UpgradeType::Effect];

        if (hero_class == HeroClass::Elf && weapon_level < 2) || weapon_level < 3  {
            upgrade_types.push(UpgradeType::Weapon);
        }

        let mut rng = rand::thread_rng();

        loop {
            if three_upgrades.len() < 4 {
                let random_index = rng.gen_range(0..upgrade_types.len());
                let upgrade_type = upgrade_types[random_index].clone();

                if upgrade_type == UpgradeType::Weapon {
                    if !three_upgrades.contains(&upgrade_type) {
                        three_upgrades.push(UpgradeType::Weapon);
                    }
                } else {
                    three_upgrades.push(upgrade_type);
                }
            } else {
                break;
            }
        }

        three_upgrades
    }
}
