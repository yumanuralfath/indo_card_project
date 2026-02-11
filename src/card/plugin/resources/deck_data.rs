use bevy::prelude::*;

/// Single card data
#[derive(Clone)]
pub struct CardData {
    pub id: u32,
    pub name: String,
}

/// Deck resource containing all cards
#[derive(Resource)]
pub struct DeckData {
    pub cards: Vec<CardData>,
}

impl Default for DeckData {
    fn default() -> Self {
        Self::new()
    }
}

impl DeckData {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        // Generate 30 test cards
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
