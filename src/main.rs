use ::bevy::{math::ops, prelude::*};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component)]
struct SnapToPlayer;

#[derive(Component)]
struct RotateToPlayer {
    rotation_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                player_movement_system,
                snap_to_player_system,
                rotate_to_player_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_a_handle: Handle<Image> = asset_server.load("enemy_A.png");
    let ship_handle: Handle<Image> = asset_server.load("ship_C.png");
    let enemy_b_handle: Handle<Image> = asset_server.load("enemy_B.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    commands.spawn((
        Sprite::from_image(ship_handle),
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
        },
    ));

    commands.spawn((
        Sprite::from_image(enemy_a_handle.clone()),
        Transform::from_xyz(0. - horizontal_margin, 0., 0.),
        SnapToPlayer,
    ));

    commands.spawn((
        Sprite::from_image(enemy_a_handle),
        Transform::from_xyz(0., 0. - vertical_margin, 0.),
        SnapToPlayer,
    ));

    commands.spawn((
        Sprite::from_image(enemy_b_handle.clone()),
        Transform::from_xyz(0. + horizontal_margin, 0., 0.),
        RotateToPlayer {
            rotation_speed: f32::to_radians(90.),
        },
    ));

    commands.spawn((
        Sprite::from_image(enemy_b_handle),
        Transform::from_xyz(0., 0. + vertical_margin, 0.),
        RotateToPlayer {
            rotation_speed: f32::to_radians(90.),
        },
    ));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.into_inner();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    }

    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_secs());

    let movement_direction = transform.rotation * Vec3::Y;

    let movement_distance = movement_factor * ship.movement_speed * time.delta_secs();

    let translation_delta = movement_direction * movement_distance;

    transform.translation += translation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.));

    transform.translation = transform.translation.min(extents).max(-extents);
}

fn snap_to_player_system(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_translation = player_transform.translation.xy();

    for mut enemy_transform in &mut query {
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

        enemy_transform.rotation = rotate_to_player;
    }
}

fn rotate_to_player_system(
    time: Res<Time>,
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
        // Get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // Get the dot product between the enemy forward vector and the direction to the player.
        let forward_dot_player = enemy_forward.dot(to_player);

        // If the dot product is approximately 1.0 then the enemy is already facing the player and
        // we can early out.
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        // Get the right vector of the enemy ship in 2D (already unit length)
        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        // Get the dot product of the enemy right vector and the direction to the player ship.
        // If the dot product is negative them we need to rotate counter clockwise, if it is
        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
        // with the right vector).
        let right_dot_player = enemy_right.dot(to_player);

        // Determine the sign of rotation from the right dot player. We need to negate the sign
        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
        // negative is clockwise.
        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        // Limit rotation so we don't overshoot the target. We need to convert our dot product to
        // an angle here so we can get an angle of rotation to clamp against.
        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0)); // Clamp acos for safety

        // Calculate angle of rotation with limit
        let rotation_angle =
            rotation_sign * (config.rotation_speed * time.delta_secs()).min(max_angle);

        // Rotate the enemy to face the player
        enemy_transform.rotate_z(rotation_angle);
    }
}
