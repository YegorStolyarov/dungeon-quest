#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ApplicationState {
    MainMenu,
    DemosMenu,
    SettingMenu, // InGame,
                 // Paused
}
