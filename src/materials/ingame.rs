use bevy::prelude::*;
use crate::materials::bullets::BulletsMaterials;
use crate::materials::dungeon::DungeonMaterials;
use crate::materials::hearts::HeartsMaterials;
use crate::materials::heroes::HeroesMaterials;
use crate::materials::monsters::MonstersMaterials;
use crate::materials::potions::PotionsMaterials;
use crate::materials::weapons::WeaponsMaterials;

#[derive(Resource)]
pub struct InGameMaterials {
    pub heroes_materials: HeroesMaterials,
    pub weapons_materials: WeaponsMaterials,
    pub dungeon_materials: DungeonMaterials,
    pub hearts_materials: HeartsMaterials,
    pub bullet_materials: BulletsMaterials,
    pub monsters_materials: MonstersMaterials,
    pub potions_materials: PotionsMaterials,
}
