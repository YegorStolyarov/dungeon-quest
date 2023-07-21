use bevy::prelude::*;

use crate::resources::language::Language;

#[derive(Resource)]
pub struct FontMaterials {
    pub roboto_font: Handle<Font>,
    pub fibberish_font: Handle<Font>,
}

impl FontMaterials {
    pub fn get_font(&self, language: Language) -> Handle<Font> {
        return match language {
            Language::VI => self.roboto_font.clone(),
            Language::EN => self.fibberish_font.clone(),
        };
    }
}
