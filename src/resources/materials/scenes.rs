use bevy::prelude::*;

use crate::resources::materials::ingame::heros::HerosMaterials;

pub struct ScenesMaterials {
    pub menu_box_materials: MenuBoxMaterials,
    pub book_tileset: Handle<Image>,
    pub home_icon_normal: Handle<Image>,
    pub home_icon_hovered: Handle<Image>,
    pub home_icon_clicked: Handle<Image>,
    pub heros_materials: HerosMaterials,
}

#[derive(Clone)]
pub struct MenuBoxMaterials {
    pub top_right: Handle<Image>,
    pub top_center: Handle<Image>,
    pub top_left: Handle<Image>,
    pub mid_right: Handle<Image>,
    pub mid_center: Handle<Image>,
    pub mid_left: Handle<Image>,
    pub bottom_right: Handle<Image>,
    pub bottom_center: Handle<Image>,
    pub bottom_left: Handle<Image>,
}
