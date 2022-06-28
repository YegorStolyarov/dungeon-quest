use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::*;

use crate::resources::glossary::Glossary;
use crate::resources::language::Language;
use crate::resources::setting::Setting;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Dictionary {
    vi_glossary: Glossary,
    vi_font: String,
    en_glossary: Glossary,
    en_font: String,
    current_language: Language,
}

impl Dictionary {
    pub fn new(current_language: Language) -> Self {
        Dictionary {
            vi_glossary: Glossary::new(Language::VI),
            vi_font: ROBOTO_FONT.to_string(),
            en_glossary: Glossary::new(Language::EN),
            en_font: FIBBERISH_FONT.to_string(),
            current_language,
        }
    }

    pub fn get_glossary(&self) -> Glossary {
        return match self.current_language {
            Language::VI => self.vi_glossary.clone(),
            Language::EN => self.en_glossary.clone(),
        };
    }

    pub fn _get_current_language(&self) -> Language {
        self.current_language.clone()
    }

    pub fn get_font(&self) -> &str {
        return match self.current_language {
            Language::VI => self.vi_font.as_str(),
            Language::EN => self.en_font.as_str(),
        };
    }
}

impl FromWorld for Dictionary {
    fn from_world(world: &mut World) -> Self {
        let setting = world.get_resource_mut::<Setting>().unwrap();
        Dictionary::new(setting.get_language())
    }
}
