use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: SNAKE_HEAD_COLOR,
            custom_size: Some(Vec2::splat(10.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
        SnakeHead,
    ));
}

fn snake_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in &mut head_positions {
        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.0;
        }

        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.0;
        }

        if keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.0;
        }

        if keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.0;
        }
    }
}
