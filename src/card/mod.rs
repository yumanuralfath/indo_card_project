mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
pub use systems::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_deck, draw_initial_hand).chain())
            .add_systems(
                Update,
                (
                    process_draw_queue,
                    animate_card_draw,
                    card_hover_system,
                    update_card_hover_visual,
                    deck_click_system,
                    card_click_system,
                    update_deck_count,
                    reindex_hand,
                    update_hand_layout,
                )
                    .chain(),
            );
    }
}
