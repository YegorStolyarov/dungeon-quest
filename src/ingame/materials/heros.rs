use bevy::prelude::*;

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
