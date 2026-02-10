use bevy::app::{App, Startup};
use indo_card_project::{CardPlugin, WindowConfigPlugin, setup_camera};

fn main() {
    App::new()
        .add_plugins(WindowConfigPlugin)
        .add_plugins(CardPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}
