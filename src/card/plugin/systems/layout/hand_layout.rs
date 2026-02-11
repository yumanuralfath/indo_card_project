use crate::card::components::*;
use crate::card::resources::*;
use crate::core::*;
use bevy::prelude::*;

/// Update card positions in hand
pub fn update_hand_layout(
    mut query: Query<(&InHand, &mut Transform, Option<&DrawAnimation>, &Hoverable), With<Card>>,
    card_size: Res<CardSize>,
) {
    let mut cards: Vec<_> = query.iter_mut().collect();
    cards.sort_by_key(|(in_hand, _, _, _)| in_hand.index);

    let count = cards.len();
    if count == 0 {
        return;
    }

    let layout = calculate_hand_layout(count, card_size.width);

    for (i, (_in_hand, mut transform, anim, hoverable)) in cards.into_iter().enumerate() {
        if anim.is_none() {
            let pos = layout.get_position(i, hoverable.is_hovered);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

/// Re-index cards after one is removed
pub fn reindex_hand(mut card_query: Query<&mut InHand, With<Card>>) {
    let mut cards: Vec<Mut<InHand>> = card_query.iter_mut().collect();
    cards.sort_by_key(|in_hand| in_hand.index);

    for (i, mut in_hand) in cards.into_iter().enumerate() {
        in_hand.index = i;
    }
}

struct HandLayout {
    start_x: f32,
    card_width: f32,
}

impl HandLayout {
    fn get_position(&self, index: usize, is_hovered: bool) -> Vec2 {
        let x = self.start_x + (index as f32 * (self.card_width + HAND_SPACING));
        let y = if is_hovered {
            HAND_Y_POSITION + HOVER_OFFSET_Y
        } else {
            HAND_Y_POSITION
        };
        Vec2::new(x, y)
    }
}

fn calculate_hand_layout(card_count: usize, card_width: f32) -> HandLayout {
    let total_width = (card_count as f32 * card_width) + ((card_count - 1) as f32 * HAND_SPACING);
    let start_x = -total_width / 2.0 + card_width / 2.0;

    HandLayout {
        start_x,
        card_width,
    }
}
