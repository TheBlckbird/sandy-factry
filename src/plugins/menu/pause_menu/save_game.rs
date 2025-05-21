use bevy_pkv::PkvStore;

use crate::{plugins::world::Seed, save_keys::SaveKey};

pub fn save_game(mut pkv: &mut PkvStore, seed: &Seed) {
    pkv.set(SaveKey::Seed, seed);
}
