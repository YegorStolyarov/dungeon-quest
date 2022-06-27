use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;

use crate::resources::language::Language;
use crate::resources::setting::Setting;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationDictionary {
    vi_dictionary: Option<Dictionary>,
    en_dictionary: Option<Dictionary>,
    current_language: Language,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Dictionary {
    pub main_menu_text: MenuText,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct MenuText {
    pub play: String,
    pub highscore: String,
    pub options: String,
    pub help: String,
    pub credits: String,
    pub quit: String,
}

impl ApplicationDictionary {
    pub fn new(current_language: Language) -> Self {
        ApplicationDictionary {
            vi_dictionary: None,
            en_dictionary: None,
            current_language,
        }
    }

    pub fn get_dictionary(&self) -> Dictionary {
        return match self.current_language {
            Language::VI => self.vi_dictionary.clone().unwrap(),
            Language::EN => self.en_dictionary.clone().unwrap(),
        };
    }

    pub fn load_language(&mut self, language: Language) {
        let file_name = match language {
            Language::VI => VIETNAMESE_LANGUAGE_FILE,
            Language::EN => ENGLISH_LANGUAGE_FILE,
        };

        match File::open(file_name) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Error read file");
                let application_text =
                    serde_json::from_str(&contents).expect("JSON was not well-formatted");

                dbg!(&application_text);

                match language {
                    Language::VI => self.vi_dictionary = Some(application_text),
                    Language::EN => self.en_dictionary = Some(application_text),
                };
            }
            Err(err) => {
                dbg!(err);
                panic!("Can't find language file");
            }
        }
    }
}

impl FromWorld for ApplicationDictionary {
    fn from_world(world: &mut World) -> Self {
        let setting = world.get_resource_mut::<Setting>().unwrap();
        dbg!(&setting);
        let mut application_dictionary = ApplicationDictionary::new(setting.get_language());

        application_dictionary.load_language(Language::EN);
        application_dictionary.load_language(Language::VI);

        application_dictionary
    }
}
