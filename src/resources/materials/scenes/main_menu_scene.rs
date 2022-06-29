use bevy::prelude::*;

pub struct MainMenuSceneMaterials {
    pub main_menu_box_materials: MainMenuBoxMaterials,
}

#[derive(Clone)]
pub struct MainMenuBoxMaterials {
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
