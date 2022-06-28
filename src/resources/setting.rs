use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;
use crate::resources::language::Language;

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Setting {
    enable_sound: bool,
    enable_music: bool,
    language: Language,
}

impl Setting {
    pub fn new(enable_sound: bool, enable_music: bool) -> Self {
        Setting {
            enable_sound,
            enable_music,
            language: Language::EN,
        }
    }

    pub fn get_enable_sound(&self) -> bool {
        self.enable_sound
    }

    pub fn get_enable_music(&self) -> bool {
        self.enable_music
    }

    pub fn get_language(&self) -> Language {
        self.language
    }

    pub fn set_enable_sound(&mut self, enable_sound: bool) {
        self.enable_sound = enable_sound;
    }

    pub fn set_enable_music(&mut self, enable_music: bool) {
        self.enable_music = enable_music;
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn store(&self) {
        let mut setting_file = File::create(SETTING_FILE).expect("Can't open setting file");
        let setting_str: String = serde_json::to_string(&self).unwrap();
        dbg!(&setting_str);
        setting_file
            .write(setting_str.as_bytes())
            .expect("Unable to write file");
    }

    pub fn load_setting(&mut self) {
        let setting: Setting;
        match File::open(SETTING_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Error read file");
                setting = serde_json::from_str(&contents).expect("JSON was not well-formatted");
            }
            Err(err) => {
                dbg!(err);
                let mut setting_file =
                    File::create(SETTING_FILE).expect("Error create setting file");
                setting = Setting::new(true, true);
                let setting_str: String = serde_json::to_string(&setting).unwrap();
                setting_file
                    .write(setting_str.as_bytes())
                    .expect("Unable to write file");
            }
        }
        self.enable_sound = setting.enable_sound;
        self.enable_music = setting.enable_music;
        self.language = setting.language;
    }
}

impl FromWorld for Setting {
    fn from_world(_world: &mut World) -> Self {
        let mut setting: Setting = Setting::new(false, false);
        setting.load_setting();
        dbg!(&setting);
        setting
    }
}
