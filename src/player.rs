use bevy::prelude::*;

use crate::physics::{Acceleration, Body, Cleanup, Velocity, GRAVITY};

const PLAYER_SIZE: f32 = 96.0;

const BULLET_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Player {
    speed: f32,
    aim: Vec2,
}

pub fn init(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(960.0, PLAYER_SIZE / 2.0, 0.0),
            ..Default::default()
        })
        .insert(Acceleration(Vec2::new(0.0, GRAVITY)))
        .insert(Velocity::default())
        .insert(Body::default())
        .insert(Player { speed: 1024.0, aim: Vec2::new(1.0, 0.0) });
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &Body)>,
) {
    let (mut velocity, mut player, body) = query.single_mut();

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }

    velocity.0.x = direction.x * player.speed;

    if direction != Vec2::ZERO {
        player.aim.x = direction.x;
    }

    player.aim.y = direction.y;

    if input.pressed(KeyCode::Z) && body.bottom {
        velocity.0.y = 2048.0;
    } else if !input.pressed(KeyCode::Z) && !body.bottom && velocity.0.y > 0.0 {
        velocity.0.y = 0.0;
    }
}

pub fn shooting(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Player)>,
) {
    let (transform, player) = query.single();

    if input.just_pressed(KeyCode::X) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ),
                ..Default::default()
            })
            .insert(Velocity(player.aim.normalize() * 2048.0))
            .insert(Cleanup);
    }
}
