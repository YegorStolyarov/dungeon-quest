use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;
use crate::resources::language::Language;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Glossary {
    pub main_menu_scene_text: MainMenuSceneText,
    pub loading_scene_text: LoadingSceneText,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct MainMenuSceneText {
    pub play: String,
    pub highscore: String,
    pub options: String,
    pub help: String,
    pub credits: String,
    pub quit: String,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct LoadingSceneText {
    pub loading: String,
}

impl Glossary {
    pub fn new(language: Language) -> Self {
        let file_name = match language {
            Language::VI => VIETNAMESE_LANGUAGE_FILE,
            Language::EN => ENGLISH_LANGUAGE_FILE,
        };

        match File::open(file_name) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Error read file");
                let glossary =
                    serde_json::from_str(&contents).expect("JSON was not well-formatted");
                dbg!(&glossary);
                glossary
            }
            Err(err) => {
                dbg!(err);
                panic!("Can't find language file");
            }
        }
    }
}
