use std::fs::File;
use std::io::prelude::*;

use crate::config::EFFECTS_FILE;
use crate::ingame::resources::effect::Effect;

pub struct EffectController {
    pub player_effects: Vec<Effect>,
}

impl EffectController {
    pub fn new() -> EffectController {
        let effect: Vec<Effect> = match File::open(EFFECTS_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let effects = serde_json::from_str(&contents).expect("Can't convert effects.json");
                effects
            }
            Err(err) => panic!("Can't find language file: {}", err.to_string()),
        };

        EffectController {
            player_effects: effect,
        }
    }
}
