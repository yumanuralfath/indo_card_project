use bevy::app::{App, Startup};
use indo_card_project::{CardPlugin, WindowConfigPlugin, setup_camera};

fn main() {
    App::new()
        .add_plugins(WindowConfigPlugin)
        .add_systems(Startup, setup_camera)
        .add_plugins(CardPlugin)
        .run();
}
