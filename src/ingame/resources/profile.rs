use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::gender::Gender;
use crate::ingame::resources::fixed::hero_class::HeroClass;
use crate::ingame::resources::game_mode::GameMode;
use crate::scenes::hero_select_scene::HeroSelectSceneButton;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub game_mode: GameMode,
    pub hero_class: HeroClass,
    pub gender: Gender,
    pub total_killed_monsters: usize,
    pub total_cleared_rooms: usize,
    pub total_cleared_waves: usize,
    pub date: String,
    pub playtime: u64,
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            name: String::new(),
            game_mode: GameMode::ClassicMode,
            hero_class: HeroClass::Elf,
            gender: Gender::Male,
            total_cleared_rooms: 0,
            total_killed_monsters: 0,
            total_cleared_waves: 0,
            date: String::new(),
            playtime: 0,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_hero(&mut self, button: HeroSelectSceneButton) {
        match button {
            HeroSelectSceneButton::MaleElf => {
                self.hero_class = HeroClass::Elf;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleElf => {
                self.hero_class = HeroClass::Elf;
                self.gender = Gender::Female;
            }
            HeroSelectSceneButton::MaleKnight => {
                self.hero_class = HeroClass::Knight;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleKnight => {
                self.hero_class = HeroClass::Knight;
                self.gender = Gender::Female;
            }
            HeroSelectSceneButton::MaleLizard => {
                self.hero_class = HeroClass::Lizard;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleLizard => {
                self.hero_class = HeroClass::Lizard;
                self.gender = Gender::Female;
            }
            HeroSelectSceneButton::MaleWizard => {
                self.hero_class = HeroClass::Wizard;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleWizard => {
                self.hero_class = HeroClass::Wizard;
                self.gender = Gender::Female;
            }
        }
    }
}
