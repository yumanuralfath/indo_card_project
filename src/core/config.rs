use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::core::data::{RESO_HEIGHT, RESO_WIDTH};

pub struct WindowConfigPlugin;

impl Plugin for WindowConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Indo Card Project".into(),
                resolution: WindowResolution::new(RESO_WIDTH, RESO_HEIGHT),
                resizable: true, //user can change resolution
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: RESO_WIDTH as f32,
                min_height: RESO_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
