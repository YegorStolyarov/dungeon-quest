use bevy::prelude::*;

use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;

#[derive(Clone)]
pub struct HerosMaterials {
    pub male_elf: Handle<Image>,
    pub male_knight: Handle<Image>,
    pub male_wizard: Handle<Image>,
    pub male_lizard: Handle<Image>,
    pub female_elf: Handle<Image>,
    pub female_knight: Handle<Image>,
    pub female_wizard: Handle<Image>,
    pub female_lizard: Handle<Image>,
}

impl HerosMaterials {
    pub fn get_texture(&self, hero_class: HeroClass, gender: Gender) -> Handle<Image> {
        return match hero_class {
            HeroClass::Elf => match gender {
                Gender::Male => self.male_elf.clone(),
                Gender::Female => self.female_elf.clone(),
            },
            HeroClass::Knight => match gender {
                Gender::Male => self.male_knight.clone(),
                Gender::Female => self.female_knight.clone(),
            },
            HeroClass::Lizard => match gender {
                Gender::Male => self.male_lizard.clone(),
                Gender::Female => self.female_lizard.clone(),
            },
            HeroClass::Wizard => match gender {
                Gender::Male => self.male_wizard.clone(),
                Gender::Female => self.female_wizard.clone(),
            },
        };
    }
}
