#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Loading,
    Game,
}