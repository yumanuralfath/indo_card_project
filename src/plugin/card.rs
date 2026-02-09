use bevy::prelude::*;
use bevy_cards::prelude::*;

use crate::core::data::{CARD_ASPECT_RATIO, CARD_COLOR, CARD_HEIGHT_RATIO, RESO_HEIGHT};

const CARD_HEIGHT: f32 = RESO_HEIGHT as f32 * CARD_HEIGHT_RATIO;
const CARD_WIDTH: f32 = CARD_HEIGHT * CARD_ASPECT_RATIO;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyCardsPlugin::new(CARD_WIDTH, CARD_HEIGHT))
            .add_systems(Startup, setup_cards);
    }
}

fn setup_cards(mut commands: Commands) {
    // spawn a green card
    commands.spawn((
        Card,
        Draggable, // to drag & drop the card, we need to add the "Draggable" component
        Sprite {
            color: CARD_COLOR,
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));

    commands.spawn((
        Card,
        Draggable, // to drag & drop the card, we need to add the "Draggable" component
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));

    // spawn a blue card, thats somewhat smaller
    commands.spawn((
        Card,
        Draggable,
        CardSize(Vec2::new(CARD_WIDTH * 0.8, CARD_HEIGHT * 0.8)),
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(CARD_WIDTH * 0.8, CARD_HEIGHT * 0.8)),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
}
