use bevy::prelude::*;

use crate::materials::flag::FlagMaterials;
use crate::materials::heroes::HeroesMaterials;
use crate::materials::icon::IconMaterials;
use crate::materials::menu_box::MenuBoxMaterials;

#[derive(Resource)]
pub struct ScenesMaterials {
    pub main_background_image: Handle<Image>,
    pub sub_background_image: Handle<Image>,
    pub menu_box_materials: MenuBoxMaterials,
    pub heroes_materials: HeroesMaterials,
    pub icon_materials: IconMaterials,
    pub flag_materials: FlagMaterials,
    pub book_tileset: Handle<Image>,
}
