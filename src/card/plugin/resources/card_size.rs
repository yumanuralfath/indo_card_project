use crate::core::*;
use bevy::prelude::*;

/// Card dimensions resource
#[derive(Resource, Clone, Copy)]
pub struct CardSize {
    pub width: f32,
    pub height: f32,
}

impl CardSize {
    pub fn from_screen() -> Self {
        let height = RESO_HEIGHT as f32 * CARD_HEIGHT_RATIO;
        let width = height * CARD_ASPECT_RATIO;
        Self { width, height }
    }

    pub fn half_size(&self) -> Vec2 {
        Vec2::new(self.width / 2.0, self.height / 2.0)
    }
}
