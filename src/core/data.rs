use bevy::color::Color;

pub const RESO_WIDTH: u32 = 1280;
pub const RESO_HEIGHT: u32 = 720;
pub const CARD_HEIGHT_RATIO: f32 = 0.2; //20 % from screen
pub const CARD_ASPECT_RATIO: f32 = 100.0 / 144.0; //Width/height

// Card Colors
pub const CARD_BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.85);
pub const CARD_BORDER_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

// Deck Positiooning
pub const DECK_POSITION_X: f32 = -400.0;
pub const DECK_POSITION_Y: f32 = 0.0;

//Hand Positioning
pub const HAND_Y_POSITION: f32 = -200.0;
pub const HAND_SPACING: f32 = 20.0;
pub const HOVER_OFFSET_Y: f32 = 30.0;

//Animation
pub const CARD_DRAW_DURATION: f32 = 0.5;
pub const CARD_DRAW_DELAY: f32 = 0.15;
