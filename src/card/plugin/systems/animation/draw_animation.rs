use crate::card::components::*;
use bevy::prelude::*;

/// Animate cards being drawn from deck to hand
pub fn animate_card_draw(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut DrawAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());

        let progress = anim.timer.fraction();
        // Ease out cubic for smooth animation
        let eased = 1.0 - (1.0 - progress).powi(3);

        transform.translation = anim.start_pos.lerp(anim.target_pos, eased);

        if anim.timer.is_finished() {
            transform.translation = anim.target_pos;
            commands.entity(entity).remove::<DrawAnimation>();
        }
    }
}
