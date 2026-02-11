use crate::card::components::*;
use crate::card::resources::*;
use crate::core::*;
use bevy::prelude::*;

/// Setup visual deck and initialize resources
pub fn setup_deck(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize resources
    let deck_data = DeckData::new();
    let initial_count = deck_data.remaining();
    let card_size = CardSize::from_screen();
    let card_assets = CardAssets::load(&asset_server);

    commands.insert_resource(deck_data);
    commands.insert_resource(card_size);
    commands.insert_resource(card_assets);

    spawn_deck_visual(&mut commands, initial_count, card_size);
}

/// Spawn the visual deck entity
fn spawn_deck_visual(commands: &mut Commands, card_count: usize, card_size: CardSize) {
    commands
        .spawn((
            Deck,
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(card_size.width, card_size.height)),
                ..default()
            },
            Transform::from_xyz(DECK_POSITION_X, DECK_POSITION_Y, 0.0),
        ))
        .with_children(|parent| {
            spawn_deck_border(parent, card_size);
            spawn_deck_counter(parent, card_count);
        });
}

fn spawn_deck_border(parent: &mut ChildSpawnerCommands, card_size: CardSize) {
    parent.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.2),
            custom_size: Some(Vec2::new(card_size.width + 4.0, card_size.height + 4.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.1),
    ));
}

fn spawn_deck_counter(parent: &mut ChildSpawnerCommands, count: usize) {
    parent.spawn((
        Text2d::new(format!("{}", count)),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

/// Queue initial hand draw
pub fn draw_initial_hand(mut commands: Commands) {
    commands.insert_resource(DrawQueue::new(5, CARD_DRAW_DELAY));
}
