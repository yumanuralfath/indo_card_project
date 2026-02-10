use bevy::prelude::*;

/// Marker for card component
#[derive(Component)]
pub struct Card {
    pub id: u32,
    pub name: String,
}

/// Size Card marker
#[derive(Resource, Clone, Copy)]
pub struct CardSize {
    pub width: f32,
    pub height: f32,
}

/// Marker for Deck
#[derive(Component)]
pub struct Deck;

/// Marker for in hand card
#[derive(Component)]
pub struct InHand {
    pub index: usize,
}

/// Resource Deck
#[derive(Resource)]
pub struct DeckData {
    pub cards: Vec<CardData>,
}

/// Card Data
#[derive(Clone)]
pub struct CardData {
    pub id: u32,
    pub name: String,
}

/// Component for draw card animation
#[derive(Component)]
pub struct DrawAnimation {
    pub timer: Timer,
    pub start_pos: Vec3,
    pub target_pos: Vec3,
}

/// Component border card as child entity
#[derive(Component)]
pub struct CardBorder;

/// Component for hover state
#[derive(Component)]
pub struct Hoverable {
    pub is_hovered: bool,
}

/// Queue Resource for draw
#[derive(Resource)]
pub struct DrawQueue {
    pub timer: Timer,
    pub cards_to_draw: usize,
}

impl DrawQueue {
    pub fn new(count: usize, delay: f32) -> Self {
        Self {
            timer: Timer::from_seconds(delay, TimerMode::Repeating),
            cards_to_draw: count,
        }
    }
}

impl Default for DeckData {
    fn default() -> Self {
        Self::new()
    }
}

impl DeckData {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        // Test for 30 card
        for i in 1..=30 {
            cards.push(CardData {
                id: i,
                name: format!("Card {}", i),
            });
        }
        Self { cards }
    }

    pub fn draw(&mut self) -> Option<CardData> {
        self.cards.pop()
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}
