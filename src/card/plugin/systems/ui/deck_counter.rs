use crate::card::components::*;
use crate::card::resources::*;
use bevy::prelude::*;

/// Update deck count display when deck changes
pub fn update_deck_count(
    deck_data: Res<DeckData>,
    deck_query: Query<&Children, With<Deck>>,
    mut text_query: Query<&mut Text2d>,
) {
    if !deck_data.is_changed() {
        return;
    }

    for children in deck_query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                **text = format!("{}", deck_data.remaining());
            }
        }
    }
}
