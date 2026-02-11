use crate::card::components::*;
use crate::card::resources::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Click on card to return it to deck
pub fn card_click_system(
    mut commands: Commands,
    mut deck_data: ResMut<DeckData>,
    card_query: Query<(Entity, &Transform, &Sprite, &Card), With<Hoverable>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    for (entity, transform, sprite, card) in card_query.iter() {
        if is_clicked(world_pos, transform, sprite) {
            return_card_to_deck(&mut deck_data, card);
            commands.entity(entity).despawn();
            break;
        }
    }
}

fn is_clicked(point: Vec2, transform: &Transform, sprite: &Sprite) -> bool {
    let Some(size) = sprite.custom_size else {
        return false;
    };

    let half_size = size / 2.0;
    let card_pos = transform.translation.truncate();

    point.x >= card_pos.x - half_size.x
        && point.x <= card_pos.x + half_size.x
        && point.y >= card_pos.y - half_size.y
        && point.y <= card_pos.y + half_size.y
}

fn return_card_to_deck(deck_data: &mut DeckData, card: &Card) {
    deck_data.cards.push(CardData {
        id: card.id,
        name: card.name.clone(),
    });
}
