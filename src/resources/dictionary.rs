use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::glossary::Glossary;
use crate::resources::language::Language;
use crate::resources::setting::Setting;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Dictionary {
    vi_glossary: Glossary,
    en_glossary: Glossary,
    current_language: Language,
}

impl Dictionary {
    pub fn new(current_language: Language) -> Self {
        Dictionary {
            vi_glossary: Glossary::new(Language::VI),
            en_glossary: Glossary::new(Language::EN),
            current_language,
        }
    }

    pub fn get_glossary(&self) -> Glossary {
        return match self.current_language {
            Language::VI => self.vi_glossary.clone(),
            Language::EN => self.en_glossary.clone(),
        };
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language.clone()
    }
}

impl FromWorld for Dictionary {
    fn from_world(world: &mut World) -> Self {
        let setting = world.get_resource_mut::<Setting>().unwrap();
        Dictionary::new(setting.get_language())
    }
}
