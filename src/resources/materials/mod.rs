use bevy::prelude::*;

use crate::resources::language::Language;

pub mod scenes;

pub struct GlobalMaterials {
    pub roboto_font: Handle<Font>,
    pub fibberish_font: Handle<Font>,
    pub main_menu_background: Handle<Image>,
    pub sub_menu_background: Handle<Image>,
    pub main_menu_scene_materials: scenes::main_menu_scene::MainMenuSceneMaterials,
    // highscore_scene_materials: scenes::main_menu_scene_materials::MainMenuSceneMaterials,
}

impl GlobalMaterials {
    pub fn get_font(&self, language: Language) -> Handle<Font> {
        return match language {
            Language::VI => self.roboto_font.clone(),
            Language::EN => self.fibberish_font.clone(),
        };
    }
}
