use bevy::prelude::*;

use crate::resources::monster::monster_class::MonsterClass;

#[derive(Clone)]
pub struct MonstersMaterials {
    pub small_zombie: Handle<Image>,
    pub zombie: Handle<Image>,
    pub big_zombie: Handle<Image>,
    pub goblin: Handle<Image>,
    pub orc: Handle<Image>,
    pub ogre: Handle<Image>,
    pub imp: Handle<Image>,
    pub chort: Handle<Image>,
    pub big_demon: Handle<Image>,
    pub swampy: Handle<Image>,
}
impl MonstersMaterials {
    pub fn get_texture(&self, monster_class: MonsterClass) -> Handle<Image> {
        match monster_class {
            MonsterClass::SmallZombie => self.small_zombie.clone(),
            MonsterClass::Zombie => self.zombie.clone(),
            MonsterClass::BigZombie => self.big_zombie.clone(),
            MonsterClass::Goblin => self.goblin.clone(),
            MonsterClass::Orc => self.orc.clone(),
            MonsterClass::Ogre => self.ogre.clone(),
            MonsterClass::Imp => self.imp.clone(),
            MonsterClass::Chort => self.chort.clone(),
            MonsterClass::BigDemon => self.big_demon.clone(),
            MonsterClass::Swampy => self.swampy.clone(),
        }
    }
}
