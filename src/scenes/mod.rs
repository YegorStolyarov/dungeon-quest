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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
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
    LuckySpinScene,
    RandomRewardScene,
}
