use bevy::prelude::*;

/// Queue system for drawing cards with delay
#[derive(Resource)]
pub struct DrawQueue {
    pub timer: Timer,
    pub cards_to_draw: usize,
}

impl DrawQueue {
    pub fn new(count: usize, delay: f32) -> Self {
        Self {
            timer: Timer::from_seconds(delay, TimerMode::Repeating),
            cards_to_draw: count,
        }
    }
}
