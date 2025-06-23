pub mod auto_save;
pub mod building;
pub mod completion;
pub mod crafting;
pub mod debug_camera;
pub mod hud;
pub mod interaction;
pub mod menu;
pub mod rendering;
pub mod simulation;
pub mod world;

pub enum RenderLayer {
    Background,
    Middleground,
    Foreground,
    Items,
    SelectionMarker,
}

impl From<RenderLayer> for f32 {
    fn from(value: RenderLayer) -> Self {
        match value {
            RenderLayer::Background => 0.0,
            RenderLayer::Middleground => 1.0,
            RenderLayer::Foreground => 2.0,
            RenderLayer::Items => 3.0,
            RenderLayer::SelectionMarker => 4.0,
        }
    }
}
