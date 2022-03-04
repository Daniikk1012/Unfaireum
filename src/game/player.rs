use bevy::{prelude::*, sprite::collide_aabb};

use crate::GameState;

use super::{
    animation::{Animation, Animations, Flippable, LoadAnimation},
    enemy::{Enemy, Spawning, Score},
    entity::{GameEntity, GAME_LAYER},
    physics::{Acceleration, Body, Cleanup, Velocity, GRAVITY},
};

pub const PLAYER_HEALTH_MAX: u32 = 3;

const PLAYER_STAND_ANIMATION: usize = 0;
const PLAYER_MOVE_ANIMATION: usize = 1;
const PLAYER_SIZE: f32 = 128.0;
const PLAYER_INVINCIBILITY: f32 = 3.0;
const PLAYER_FLASH_FREQUENCY: f32 = 1.0 / 4.0;

const BULLET_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Player {
    pub damage: u32,
    pub health: u32,
    speed: f32,
    aim: Vec2,
    direction: f32,
    now: f32,
    max: f32,
}

#[derive(Component)]
pub struct Bullet;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                960.0,
                PLAYER_SIZE / 2.0,
                GAME_LAYER,
            ),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(Animations {
            animations: vec![
                Animation {
                    textures: asset_server.load_animation("player/stand", 1),
                    ..Default::default()
                },
                Animation {
                    textures: asset_server.load_animation("player/move", 2),
                    max: 1.0 / 8.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        })
        .insert(Flippable)
        .insert(Acceleration(Vec2::new(0.0, GRAVITY)))
        .insert(Velocity::default())
        .insert(Body::default())
        .insert(Player {
            damage: 0,
            health: PLAYER_HEALTH_MAX,
            speed: 768.0,
            aim: Vec2::new(1.0, 0.0),
            direction: 1.0,
            now: PLAYER_INVINCIBILITY,
            max: PLAYER_INVINCIBILITY,
        });
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &Body)>,
) {
    let (mut velocity, mut player, body) =
        if let Ok(result) = query.get_single_mut() {
            result
        } else {
            return;
        };

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
    } else {
        player.aim.x = player.direction;
    }

    if direction.x != 0.0 {
        player.direction = direction.x;
    }

    player.aim.y = direction.y;

    if input.pressed(KeyCode::Z) && body.bottom {
        velocity.0.y = 2048.0;
    } else if !input.pressed(KeyCode::Z) && !body.bottom && velocity.0.y > 0.0 {
        velocity.0.y = 0.0;
    }
}

pub fn animation(mut query: Query<(&mut Animations, &Velocity), With<Player>>) {
    let (mut animations, velocity) = if let Ok(result) = query.get_single_mut()
    {
        result
    } else {
        return;
    };

    if velocity.0.x == 0.0 && animations.current != PLAYER_STAND_ANIMATION {
        animations.current = PLAYER_STAND_ANIMATION;
    } else if velocity.0.x != 0.0 && animations.current != PLAYER_MOVE_ANIMATION
    {
        animations.current = PLAYER_MOVE_ANIMATION;
    }
}

pub fn shoot(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, &Player)>,
) {
    let (transform, player) = if let Ok(result) = query.get_single() {
        result
    } else {
        return;
    };

    if input.just_pressed(KeyCode::X) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                )
                .with_rotation(Quat::from_rotation_z(
                    player.aim.y.atan2(player.aim.x),
                )),
                texture: asset_server.load("bullet.png"),
                ..Default::default()
            })
            .insert(GameEntity)
            .insert(Velocity(player.aim.normalize() * 1536.0))
            .insert(Cleanup)
            .insert(Bullet);
    }
}

pub fn damage(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut query: Query<(Entity, &mut Player)>,
) {
    let (entity, mut player) = if let Ok(result) = query.get_single_mut() {
        result
    } else {
        return;
    };

    if player.damage > 0 && player.now >= player.max {
        if player.health > player.damage {
            player.health -= player.damage;
            player.now = 0.0;
        } else {
            state.push(GameState::GameOver).unwrap();
            commands.entity(entity).despawn();
        }
    }
}

pub fn invincibility(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Sprite)>,
) {
    let (mut player, mut sprite) = if let Ok(result) = query.get_single_mut() {
        result
    } else {
        return;
    };

    if player.now < player.max {
        player.now += time.delta_seconds();

        if player.now >= player.max {
            player.damage = 0;
            sprite.color.set_a(1.0);
        } else {
            let m =
                player.now % PLAYER_FLASH_FREQUENCY / PLAYER_FLASH_FREQUENCY;
            if m < 0.5 {
                sprite.color.set_a(1.0 - m);
            } else {
                sprite.color.set_a(m);
            }
        }
    }
}

pub fn bullet(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    mut enemy_query: Query<
        (Entity, &mut Enemy, &Transform, &Sprite),
        Without<Spawning>,
    >,
) {
    for (bullet_entity, bullet_transform, bullet_sprite) in bullet_query.iter()
    {
        for (enemy_entity, mut enemy, enemy_transform, enemy_sprite) in
            enemy_query.iter_mut()
        {
            if collide_aabb::collide(
                bullet_transform.translation,
                bullet_sprite.custom_size.unwrap(),
                enemy_transform.translation,
                enemy_sprite.custom_size.unwrap(),
            )
            .is_some()
            {
                commands.entity(bullet_entity).despawn();

                if enemy.health > 1 {
                    enemy.health -= 1;
                } else {
                    score.0 += 1;
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}
