use strum_macros::AsRefStr;

pub mod save_game;

/// Keys for saved data
///
/// Currently only the saved game and its version, but in the future
/// it could include things like username or number of available saves
#[derive(AsRefStr)]
pub enum SaveKey {
    GameSave,
    Version,
}
