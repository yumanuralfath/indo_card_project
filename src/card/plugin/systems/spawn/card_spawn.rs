use crate::card::components::*;
use crate::card::resources::*;
use crate::core::*;
use bevy::prelude::*;

/// Spawns a card with draw animation
pub fn spawn_card(
    commands: &mut Commands,
    card_data: CardData,
    hand_index: usize,
    card_size: CardSize,
    font: Handle<Font>,
) {
    let target_pos = calculate_hand_position(hand_index, card_size.width);

    let card_entity = spawn_card_entity(commands, &card_data, hand_index, target_pos, card_size);
    let border_entity = spawn_card_border(commands, card_size);
    let text_entity = spawn_card_text(commands, &card_data.name, font);

    commands
        .entity(card_entity)
        .add_children(&[border_entity, text_entity]);
}

fn spawn_card_entity(
    commands: &mut Commands,
    card_data: &CardData,
    index: usize,
    target_pos: Vec3,
    card_size: CardSize,
) -> Entity {
    commands
        .spawn((
            Card {
                id: card_data.id,
                name: card_data.name.clone(),
            },
            InHand { index },
            Hoverable { is_hovered: false },
            DrawAnimation {
                timer: Timer::from_seconds(CARD_DRAW_DURATION, TimerMode::Once),
                start_pos: Vec3::new(DECK_POSITION_X, DECK_POSITION_Y, 0.0),
                target_pos,
            },
            Sprite {
                color: CARD_BG_COLOR,
                custom_size: Some(Vec2::new(card_size.width, card_size.height)),
                ..Default::default()
            },
            Transform::from_xyz(DECK_POSITION_X, DECK_POSITION_Y, 1.0),
        ))
        .id()
}

fn spawn_card_border(commands: &mut Commands, card_size: CardSize) -> Entity {
    commands
        .spawn((
            CardBorder,
            Sprite {
                color: CARD_BORDER_COLOR,
                custom_size: Some(Vec2::new(card_size.width + 4.0, card_size.height + 4.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -0.1),
        ))
        .id()
}

fn spawn_card_text(commands: &mut Commands, text: &str, font: Handle<Font>) -> Entity {
    commands
        .spawn((
            Text2d::new(text),
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Transform::from_xyz(0.0, 0.0, 0.2),
        ))
        .id()
}

/// Process the draw queue and spawn cards
pub fn process_draw_queue(
    mut commands: Commands,
    mut deck_data: ResMut<DeckData>,
    draw_queue: Option<ResMut<DrawQueue>>,
    time: Res<Time>,
    hand_query: Query<&InHand>,
    card_size: Res<CardSize>,
    card_assets: Res<CardAssets>,
) {
    let Some(mut queue) = draw_queue else {
        return;
    };

    if queue.cards_to_draw == 0 {
        commands.remove_resource::<DrawQueue>();
        return;
    }

    queue.timer.tick(time.delta());

    if queue.timer.just_finished() {
        let current_hand_size = hand_query.iter().count();

        if let Some(card_data) = deck_data.draw() {
            spawn_card(
                &mut commands,
                card_data,
                current_hand_size,
                *card_size,
                card_assets.font.clone(),
            );
            queue.cards_to_draw -= 1;
        } else {
            queue.cards_to_draw = 0;
        }
    }
}

/// Calculate hand position for a card at given index
fn calculate_hand_position(_index: usize, _card_width: f32) -> Vec3 {
    Vec3::new(0.0, HAND_Y_POSITION, 1.0)
}
