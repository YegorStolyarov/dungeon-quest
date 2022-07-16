pub mod dungeon;
pub mod heros;
pub mod weapons;

pub struct InGameMaterials {
    pub heros_materials: heros::HerosMaterials,
    pub weapons_materials: weapons::WeaponsMaterials,
    pub dungeon_materials: dungeon::DungeonMaterials,
}
