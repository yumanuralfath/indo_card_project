use crate::card::plugin::card_click::card_click_system;
use crate::card::plugin::card_spawn::process_draw_queue;
use crate::card::plugin::deck_click::deck_click_system;
use crate::card::plugin::deck_spawn::draw_initial_hand;
use crate::card::plugin::deck_spawn::setup_deck;
use crate::card::plugin::hover::card_hover_system;
use crate::card::plugin::hover::update_card_hover_visual;
use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use systems::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_deck, draw_initial_hand).chain())
            .add_systems(
                Update,
                (
                    // Draw & Animation
                    process_draw_queue,
                    animate_card_draw,
                    // Interaction
                    card_hover_system,
                    update_card_hover_visual,
                    deck_click_system,
                    card_click_system,
                    // Layout & UI
                    reindex_hand,
                    update_hand_layout,
                    update_deck_count,
                )
                    .chain(),
            );
    }
}
