use crate::card::components::*;
use crate::card::resources::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Click on deck to draw a card
pub fn deck_click_system(
    mut commands: Commands,
    deck_query: Query<(&Transform, &Sprite), With<Deck>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    draw_queue: Option<Res<DrawQueue>>,
) {
    // Don't allow clicking if already drawing
    if draw_queue.is_some() {
        return;
    }

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

    for (transform, sprite) in deck_query.iter() {
        if is_deck_clicked(world_pos, transform, sprite) {
            commands.insert_resource(DrawQueue::new(1, 0.0));
        }
    }
}

fn is_deck_clicked(point: Vec2, transform: &Transform, sprite: &Sprite) -> bool {
    let Some(size) = sprite.custom_size else {
        return false;
    };

    let half_size = size / 2.0;
    let deck_pos = transform.translation.truncate();

    point.x >= deck_pos.x - half_size.x
        && point.x <= deck_pos.x + half_size.x
        && point.y >= deck_pos.y - half_size.y
        && point.y <= deck_pos.y + half_size.y
}
