use bevy::prelude::*;

use crate::ingame::materials::heros::HerosMaterials;

pub struct ScenesMaterials {
    pub menu_box_materials: MenuBoxMaterials,
    pub book_tileset: Handle<Image>,
    pub icon_materials: IconMaterials,
    pub heros_materials: HerosMaterials,
    pub flag_materials: FlagMaterials,
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

pub struct FlagMaterials {
    pub vietnam: Handle<Image>,
    pub united_states: Handle<Image>,
}

pub struct IconMaterials {
    pub sound_icon_off: Handle<Image>,
    pub sound_icon_on: Handle<Image>,
    pub sound_icon_hovered: Handle<Image>,
    pub music_icon_off: Handle<Image>,
    pub music_icon_on: Handle<Image>,
    pub music_icon_hovered: Handle<Image>,
    pub home_icon_hovered: Handle<Image>,
    pub home_icon_clicked: Handle<Image>,
    pub home_icon_normal: Handle<Image>,
    pub leaderboard: Handle<Image>,
    pub leaderboard_hovered: Handle<Image>,
    pub restart: Handle<Image>,
    pub restart_hovered: Handle<Image>,
}
