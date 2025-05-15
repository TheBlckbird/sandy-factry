pub mod building;
pub mod crafting;
pub mod debug_camera;
pub mod hud;
pub mod rendering;
pub mod simulation;
pub mod world;

pub enum TilemapLayer {
    Background,
    Middleground,
    Foreground,
    Items,
}

impl From<TilemapLayer> for f32 {
    fn from(value: TilemapLayer) -> Self {
        match value {
            TilemapLayer::Background => 0.0,
            TilemapLayer::Middleground => 1.0,
            TilemapLayer::Foreground => 2.0,
            TilemapLayer::Items => 3.0,
        }
    }
}
