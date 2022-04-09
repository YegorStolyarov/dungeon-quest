use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Setting {
    enable_sound: bool,
    enable_music: bool,
    fullscreen: bool,
}

pub fn load_setting(mut command: Commands) {
    match File::open(SETTING_FILE) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Error read file");
            let setting: Setting =
                serde_json::from_str(&contents).expect("JSON was not well-formatted");
            dbg!(&setting);

            command.insert_resource(setting);
        }
        Err(err) => {
            dbg!(err);
            let setting = create_new_setting_file();
            command.insert_resource(setting);
        }
    };
}

fn create_new_setting_file() -> Setting {
    let mut setting_file = File::create(SETTING_FILE).expect("Error create setting file");
    let setting = Setting {
        enable_sound: false,
        enable_music: false,
        fullscreen: false,
    };
    let setting_str: String = serde_json::to_string(&setting).unwrap();
    setting_file
        .write(setting_str.as_bytes())
        .expect("Unable to write file");

    setting
}

fn store_setting(setting: Setting) {
    let mut setting_file = File::open(SETTING_FILE).expect("Can't open setting file");
    let setting_str: String = serde_json::to_string(&setting).unwrap();
    dbg!(&setting_str);
    setting_file
        .write(setting_str.as_bytes())
        .expect("Unable to write file");
}
