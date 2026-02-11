use crate::card::components::*;
use crate::core::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Detect mouse hover on cards
pub fn card_hover_system(
    mut card_query: Query<(&Transform, &Sprite, &mut Hoverable), With<Card>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        reset_all_hover(&mut card_query);
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        reset_all_hover(&mut card_query);
        return;
    };

    for (transform, sprite, mut hoverable) in card_query.iter_mut() {
        hoverable.is_hovered = is_point_in_card(world_pos, transform, sprite);
    }
}

type CardHoverQuery<'w, 's> =
    Query<'w, 's, (&'static Hoverable, &'static Children), (With<Card>, Changed<Hoverable>)>;

type CardBorderQuery<'w, 's> = Query<'w, 's, &'static mut Sprite, With<CardBorder>>;

/// Update card visual when hover state changes
pub fn update_card_hover_visual(card_query: CardHoverQuery, mut sprite_query: CardBorderQuery) {
    for (hoverable, children) in card_query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                sprite.color = if hoverable.is_hovered {
                    Color::srgb(0.9, 0.7, 0.2) // Gold for hover
                } else {
                    CARD_BORDER_COLOR
                };
            }
        }
    }
}

fn is_point_in_card(point: Vec2, transform: &Transform, sprite: &Sprite) -> bool {
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

fn reset_all_hover(card_query: &mut Query<(&Transform, &Sprite, &mut Hoverable), With<Card>>) {
    for (_, _, mut hoverable) in card_query.iter_mut() {
        hoverable.is_hovered = false;
    }
}
