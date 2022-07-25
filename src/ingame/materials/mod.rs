pub mod dungeon;
pub mod hearts;
pub mod heros;
pub mod weapons;

pub struct InGameMaterials {
    pub heros_materials: heros::HerosMaterials,
    pub weapons_materials: weapons::WeaponsMaterials,
    pub dungeon_materials: dungeon::DungeonMaterials,
    pub hearts_materials: hearts::HeartsMaterials,
}
