use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub size: Vec2,
}

fn spawn_player(mut commands: Commands) {
    let box_size = Vec2::new(50.0, 50.0);
    commands.spawn((
        Player {
            speed: 400.0,
            size: box_size,
        },
        Sprite {
            color: Color::srgb(0.0, 0.9, 0.5), // Hijau Neon
            custom_size: Some(box_size),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_query: Query<&Projection>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    // Ambil proyeksi kamera
    let Ok(projection) = camera_query.single() else {
        return;
    };

    // Unwrap Result dari single_mut()
    let Ok((player, mut transform)) = player_query.single_mut() else {
        return;
    };

    // 1. Kalkulasi Pergerakan
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * player.speed * time.delta_secs();
    }

    // 2. Batasan (Clamping) agar tidak keluar kamera
    // Ekstrak area dari Projection
    let area = if let Projection::Orthographic(ortho) = projection {
        ortho.area
    } else {
        // Fallback jika bukan orthographic
        return;
    };

    let half_player_width = player.size.x / 2.0;
    let half_player_height = player.size.y / 2.0;

    // Batasi posisi X dan Y berdasarkan area kamera
    transform.translation.x = transform.translation.x.clamp(
        area.min.x + half_player_width,
        area.max.x - half_player_width,
    );
    transform.translation.y = transform.translation.y.clamp(
        area.min.y + half_player_height,
        area.max.y - half_player_height,
    );
}
