use bevy::prelude::*;
use std::collections::HashMap;

use crate::ingame::resources::effect::effect_information::EffectInformation;
use crate::ingame::resources::effect::effect_type::EffectType;

pub struct PlayerEffects {
    pub information: Vec<EffectInformation>,
    pub activated_effects: HashMap<EffectType, Timer>,
}

impl PlayerEffects {
    pub fn new(information: Vec<EffectInformation>) -> PlayerEffects {
        PlayerEffects {
            information,
            activated_effects: HashMap::new(),
        }
    }
}
