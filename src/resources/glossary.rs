use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;
use crate::resources::language::Language;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Glossary {
    pub highscore_scene_text: HighscoreSceneText,
    pub main_menu_scene_text: MainMenuSceneText,
    pub loading_scene_text: LoadingSceneText,
    pub options_scene_text: OptionsSceneText,
    pub result_scene_text: ResultSceneText,
    pub help_scene_text: HelpSceneText,
    pub shared_text: SharedText,
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

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct HighscoreSceneText {
    pub name: String,
    pub gender: String,
    pub game_mode: String,
    pub total_killed_monsters: String,
    pub total_cleared_rooms: String,
    pub total_cleared_waves: String,
    pub date: String,
    pub playtime: String,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct OptionsSceneText {
    pub options: String,
    pub enable_music: String,
    pub enable_sound: String,
    pub language: String,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct HelpSceneText {
    pub help: String,
    pub move_up: String,
    pub move_down: String,
    pub move_left: String,
    pub move_right: String,
    pub use_skill: String,
    pub attack: String,
    pub aim: String,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct SharedText {
    pub male: String,
    pub female: String,
    pub classic_mode: String,
    pub survival_mode: String,
    pub select_game_mode: String,
    pub select_hero: String,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct ResultSceneText {
    pub result: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub total_killed_monsters: String,
    pub total_cleared_rooms: String,
    pub total_cleared_waves: String,
    pub playtime: String,
}

impl Glossary {
    pub fn new(language: Language) -> Self {
        let file_name = match language {
            Language::VI => VIETNAMESE_LANGUAGE_FILE,
            Language::EN => ENGLISH_LANGUAGE_FILE,
        };

        match File::open(file_name) {
            Ok(mut file) => {
                let error_message = format!(
                    "{}: JSON was not well-formatted",
                    if language == Language::VI {
                        "Language::VI"
                    } else {
                        "Language::EN"
                    }
                );

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let glossary = serde_json::from_str(&contents).expect(error_message.as_str());
                glossary
            }
            Err(err) => panic!("Can't find language file: {}", err.to_string()),
        }
    }
}
