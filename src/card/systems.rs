use crate::card::components::*;
use crate::core::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Setup visual deck at screen
pub fn setup_deck(mut commands: Commands) {
    // Insert deck  data as resource
    let deck_data = DeckData::new();
    let initial_count = deck_data.remaining();
    commands.insert_resource(deck_data);

    // Calculate card size based on screen
    let card_height = RESO_HEIGHT as f32 * CARD_HEIGHT_RATIO;
    let card_width = card_height * CARD_ASPECT_RATIO;

    // set card size at setup
    commands.insert_resource(CardSize {
        width: card_width,
        height: card_height,
    });

    // Spawn visual deck
    commands
        .spawn((
            Deck,
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(card_width, card_height)),
                ..default()
            },
            Transform::from_xyz(DECK_POSITION_X, DECK_POSITION_Y, 0.0),
        ))
        .with_children(|parent| {
            // Border for Deck
            parent.spawn((
                Sprite {
                    color: Color::srgb(0.1, 0.1, 0.2),
                    custom_size: Some(Vec2::new(card_width + 4.0, card_height + 4.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, -0.1),
            ));
            //Text for display total card at deck
            parent.spawn((
                Text::new(format!("{}", initial_count)),
                TextFont {
                    font_size: 30.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
        });
}

/// Draw card with animation usign queue system
pub fn draw_initial_hand(mut commands: Commands) {
    // queue 5 card for draw with delay
    commands.insert_resource(DrawQueue::new(5, CARD_DRAW_DELAY));
}

/// Draw animation with queue system
pub fn process_draw_queue(
    mut commands: Commands,
    mut deck_data: ResMut<DeckData>,
    draw_queue: Option<ResMut<DrawQueue>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    hand_query: Query<&InHand>,
    card_size: Res<CardSize>,
) {
    if let Some(mut queue) = draw_queue {
        if queue.cards_to_draw == 0 {
            commands.remove_resource::<DrawQueue>();
            return;
        }

        queue.timer.tick(time.delta());

        if queue.timer.just_finished() {
            // calculate index card at hand
            let current_hand_size = hand_query.iter().count();

            if let Some(card_data) = deck_data.draw() {
                spawn_card_with_animation(
                    &mut commands,
                    &asset_server,
                    card_data,
                    current_hand_size,
                    *card_size,
                );
                queue.cards_to_draw -= 1;
            } else {
                // Deck Empty
                queue.cards_to_draw = 0;
            }
        }
    }
}

/// Helper function for card Spawner
fn spawn_card_with_animation(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    card_data: CardData,
    index: usize,
    card_size: CardSize,
) {
    let card_name = card_data.name.clone();
    let card_width = card_size.width;
    let card_height = card_size.height;

    // Calculate position target
    let target_pos = calculate_hand_position(index, card_width);

    // Spawn Card entity as parent
    let card_entity = commands
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
                custom_size: Some(Vec2::new(card_width, card_height)),
                ..Default::default()
            },
            Transform::from_xyz(DECK_POSITION_X, DECK_POSITION_Y, 1.0),
        ))
        .id();

    // Spawn Border as child
    let border_entity = commands
        .spawn((
            CardBorder,
            Sprite {
                color: CARD_BORDER_COLOR,
                custom_size: Some(Vec2::new(card_width + 4.0, card_height + 4.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -0.1),
        ))
        .id();

    // Spawn text at card center as child
    let text_entity = commands
        .spawn((
            Text::new(card_name),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Transform::from_xyz(0.0, 0.0, 0.2),
        ))
        .id();

    // Set Hierarchy
    commands
        .entity(card_entity)
        .add_children(&[border_entity, text_entity]);
}

/// Animate card draw
pub fn animate_card_draw(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut DrawAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());

        let progress = anim.timer.fraction();

        //Ease out cubic for smooth animation
        let eased = 1.0 - (1.0 - progress).powi(3);

        transform.translation = anim.start_pos.lerp(anim.target_pos, eased);

        if anim.timer.is_finished() {
            transform.translation = anim.target_pos;
            commands.entity(entity).remove::<DrawAnimation>();
        }
    }
}

/// Calculate hand position for card
fn calculate_hand_position(_index: usize, _card_width: f32) -> Vec3 {
    // TODO: temporary update hand
    Vec3::new(0.0, HAND_Y_POSITION, 1.0)
}

/// Update Card layout in hand
pub fn update_hand_layout(
    mut query: Query<(&InHand, &mut Transform, Option<&DrawAnimation>, &Hoverable), With<Card>>,
    card_size: Res<CardSize>,
) {
    let card_width = card_size.width;

    let mut cards: Vec<_> = query.iter_mut().collect();

    // Reindex card
    cards.sort_by_key(|(in_hand, _, _, _)| in_hand.index);

    let count = cards.len();

    if count == 0 {
        return;
    }

    // Calculate total width which needed
    let total_width = (count as f32 * card_width) + ((count - 1) as f32 * HAND_SPACING);
    let start_x = -total_width / 2.0 + card_width / 2.0;

    // Update each card position
    for (i, (_in_hand, transform, anim, hoverable)) in cards.iter_mut().enumerate() {
        let x = start_x + (i as f32 * (card_width + HAND_SPACING));
        let y = if hoverable.is_hovered {
            HAND_Y_POSITION + HOVER_OFFSET_Y
        } else {
            HAND_Y_POSITION
        };

        // Hanya update jika tidak sedang animasi draw
        if anim.is_none() {
            transform.translation.x = x;
            transform.translation.y = y;
        } else {
            // Update target position untuk animasi yang sedang berjalan
            // Ini akan dihandle oleh animate_card_draw
        }
    }
}

/// Detect hover on cards
pub fn card_hover_system(
    mut card_query: Query<(&Transform, &Sprite, &mut Hoverable), With<Card>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Gunakan .get_single() untuk keamanan agar tidak crash jika window/camera belum siap
    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        for (transform, sprite, mut hoverable) in card_query.iter_mut() {
            if let Some(size) = sprite.custom_size {
                let half_size = size / 2.0;
                let card_pos = transform.translation.truncate();

                let is_inside = world_pos.x >= card_pos.x - half_size.x
                    && world_pos.x <= card_pos.x + half_size.x
                    && world_pos.y >= card_pos.y - half_size.y
                    && world_pos.y <= card_pos.y + half_size.y;

                hoverable.is_hovered = is_inside;
            }
        }
        return; // Berhasil diproses
    }

    // Jika cursor tidak ada di window, reset semua hover
    for (_, _, mut hoverable) in card_query.iter_mut() {
        hoverable.is_hovered = false;
    }
}

/// Click on deck to draw a card
pub fn deck_click_system(
    mut commands: Commands,
    deck_query: Query<(&Transform, &Sprite), With<Deck>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    draw_queue: Option<Res<DrawQueue>>,
) {
    if draw_queue.is_some() {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        let Ok(window) = window_query.single() else {
            return;
        };
        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };

        if let Some(cursor_pos) = window.cursor_position()
            && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
        {
            for (transform, sprite) in deck_query.iter() {
                if let Some(size) = sprite.custom_size {
                    let half_size = size / 2.0;
                    let deck_pos = transform.translation.truncate();

                    if world_pos.x >= deck_pos.x - half_size.x
                        && world_pos.x <= deck_pos.x + half_size.x
                        && world_pos.y >= deck_pos.y - half_size.y
                        && world_pos.y <= deck_pos.y + half_size.y
                    {
                        commands.insert_resource(DrawQueue::new(1, 0.0));
                    }
                }
            }
        }
    }
}

/// Click card to return it to deck
pub fn card_click_system(
    mut commands: Commands,
    mut deck_data: ResMut<DeckData>,
    card_query: Query<(Entity, &Transform, &Sprite, &Card), With<Hoverable>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let Ok(window) = window_query.single() else {
            return;
        };
        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };

        if let Some(cursor_pos) = window.cursor_position()
            && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
        {
            for (entity, transform, sprite, card) in card_query.iter() {
                if let Some(size) = sprite.custom_size {
                    let half_size = size / 2.0;
                    let card_pos = transform.translation.truncate();

                    if world_pos.x >= card_pos.x - half_size.x
                        && world_pos.x <= card_pos.x + half_size.x
                        && world_pos.y >= card_pos.y - half_size.y
                        && world_pos.y <= card_pos.y + half_size.y
                    {
                        deck_data.cards.push(CardData {
                            id: card.id,
                            name: card.name.clone(),
                        });

                        commands.entity(entity).despawn();
                        break;
                    }
                }
            }
        }
    }
}

/// Update card visual on hover
pub fn update_card_hover_visual(
    card_query: Query<(&Hoverable, &Children), (With<Card>, Changed<Hoverable>)>,
    mut sprite_query: Query<&mut Sprite, With<CardBorder>>,
) {
    for (hoverable, children) in card_query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                sprite.color = if hoverable.is_hovered {
                    Color::srgb(0.9, 0.7, 0.2) // Gold color for hover
                } else {
                    CARD_BORDER_COLOR
                };
            }
        }
    }
}

/// Update deck count display
pub fn update_deck_count(
    deck_data: Res<DeckData>,
    deck_query: Query<&Children, With<Deck>>,
    mut text_query: Query<&mut Text>,
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

/// Re-index cards after one is removed
pub fn reindex_hand(mut card_query: Query<&mut InHand, With<Card>>) {
    // Gunakan .iter_mut() dan pastikan sorting benar
    let mut cards: Vec<Mut<InHand>> = card_query.iter_mut().collect();
    cards.sort_by_key(|in_hand| in_hand.index);

    for (i, mut in_hand) in cards.into_iter().enumerate() {
        in_hand.index = i;
    }
}
