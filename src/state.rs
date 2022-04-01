#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ApplicationState {
    MainMenu,
    InGame,
    Paused,
}
