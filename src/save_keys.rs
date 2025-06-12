use strum_macros::AsRefStr;

/// Keys for saved data
///
/// Currently only the saved game, but in future it could include
/// things like username or number of available saves
#[derive(AsRefStr)]
pub enum SaveKey {
    GameSave,
}
