use crate::materials::bullets::BulletsMaterials;
use crate::materials::dungeon::DungeonMaterials;
use crate::materials::hearts::HeartsMaterials;
use crate::materials::heros::HerosMaterials;
use crate::materials::weapons::WeaponsMaterials;

pub struct InGameMaterials {
    pub heros_materials: HerosMaterials,
    pub weapons_materials: WeaponsMaterials,
    pub dungeon_materials: DungeonMaterials,
    pub hearts_materials: HeartsMaterials,
    pub bullet_materials: BulletsMaterials,
}
