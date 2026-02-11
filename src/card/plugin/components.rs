use bevy::prelude::*;

/// Marker for card entity
#[derive(Component)]
pub struct Card {
    pub id: u32,
    pub name: String,
}

/// Marker for deck entity
#[derive(Component)]
pub struct Deck;

/// Marker for cards in hand
#[derive(Component)]
pub struct InHand {
    pub index: usize,
}

/// Component for draw animation state
#[derive(Component)]
pub struct DrawAnimation {
    pub timer: Timer,
    pub start_pos: Vec3,
    pub target_pos: Vec3,
}

/// Border child component
#[derive(Component)]
pub struct CardBorder;

/// Hover state component
#[derive(Component)]
pub struct Hoverable {
    pub is_hovered: bool,
}
