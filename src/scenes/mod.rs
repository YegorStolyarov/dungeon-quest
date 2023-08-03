use bevy::prelude::States;

pub mod credits_scene;
pub mod game_mode_select_scene;
pub mod help_scene;
pub mod hero_select_scene;
pub mod highscore_scene;
pub mod loading_scene;
pub mod main_menu_scene;
pub mod options_scene;
pub mod pause_scene;
pub mod result_scene;
pub mod reward_scene;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum SceneState {
    #[default]
    LoadingScene,
    MainMenuScene,
    HighscoreScene,
    OptionsScene,
    HelpScene,
    CreditsScene,
    GameModeSelectScene,
    HeroSelectScene,
    PreClassicMode,
    InGameClassicMode,
    PreSurvivalMode,
    InGameSurvivalMode,
    PauseScene,
    ResultScene,
    RewardsScene,
    RewardScene,
}
