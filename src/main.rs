use bevy::app::{App, Startup};
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use indo_card_project::*;

fn main() {
    App::new()
        .add_plugins(window_config_plugin())
        .add_plugins(CardPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn window_config_plugin() -> impl PluginGroup {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Indo Card Project".into(),
            resolution: WindowResolution::new(RESO_WIDTH, RESO_HEIGHT),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn setup_camera(mut commands: Commands) {
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

    commands.insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)));
}
