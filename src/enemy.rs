use bevy::{prelude::*, sprite::collide_aabb};
use rand::Rng;

use crate::{
    animation::Flippable,
    background::GAME_LAYER,
    camera::GameCamera,
    physics::{Acceleration, Body, Cleanup, Velocity, GRAVITY},
    player::Player,
};

const ENEMY_SIZE: f32 = 128.0;

const BULLET_SIZE: f32 = 32.0;

pub struct SpawnInterval {
    now: f32,
    min: f32,
    max: f32,
}

#[derive(Component)]
pub struct Spawning {
    now: f32,
    max: f32,
}

#[derive(Component)]
pub struct Walker {
    acceleration: f32,
}

#[derive(Component)]
pub struct Shooter {
    speed: f32,
    bullet_speed: f32,
    now: f32,
    max: f32,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
}

impl Default for SpawnInterval {
    fn default() -> Self {
        SpawnInterval { now: 0.0, min: 0.5, max: 7.5 }
    }
}

pub fn prespawn(
    mut commands: Commands,
    mut spawn_interval: ResMut<SpawnInterval>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, &OrthographicProjection), With<GameCamera>>,
) {
    let (camera_transform, projection) = query.single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    spawn_interval.now += time.delta_seconds();

    while spawn_interval.now >= spawn_interval.max {
        match rand::thread_rng().gen_range(1..=10) {
            1..=9 => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(
                                ENEMY_SIZE, ENEMY_SIZE,
                            )),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            rand::thread_rng().gen_range(
                                camera_bounds.left + ENEMY_SIZE
                                    ..camera_bounds.right - ENEMY_SIZE,
                            ),
                            rand::thread_rng().gen_range(
                                camera_bounds.bottom + ENEMY_SIZE
                                    ..camera_bounds.top - ENEMY_SIZE,
                            ),
                            GAME_LAYER,
                        ),
                        texture: asset_server.load("enemy/walker/move.png"),
                        ..Default::default()
                    })
                    .insert(Spawning { now: 0.0, max: 1.0 })
                    .insert(Enemy { health: 1 })
                    .insert(Walker { acceleration: 768.0 });
            }
            10 => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(
                                ENEMY_SIZE, ENEMY_SIZE,
                            )),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            rand::thread_rng().gen_range(
                                camera_bounds.left + ENEMY_SIZE
                                    ..camera_bounds.right - ENEMY_SIZE,
                            ),
                            rand::thread_rng().gen_range(
                                camera_bounds.bottom + ENEMY_SIZE
                                    ..camera_bounds.top - ENEMY_SIZE,
                            ),
                            GAME_LAYER,
                        ),
                        texture: asset_server.load("enemy/shooter/move.png"),
                        ..Default::default()
                    })
                    .insert(Spawning { now: 0.0, max: 1.5 })
                    .insert(Enemy { health: 2 })
                    .insert(Shooter {
                        speed: 256.0,
                        bullet_speed: 512.0,
                        now: 0.0,
                        max: 2.0,
                    });
            }
            _ => unreachable!(),
        }

        spawn_interval.now -= spawn_interval.max;
        spawn_interval.max =
            (spawn_interval.max * 0.975).max(spawn_interval.min);
    }
}

pub fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut Spawning)>,
) {
    for (entity, mut sprite, mut spawning) in query.iter_mut() {
        spawning.now += time.delta_seconds();

        if spawning.now >= spawning.max {
            sprite.color.set_a(1.0);

            commands
                .entity(entity)
                .remove::<Spawning>()
                .insert(Flippable)
                .insert(Acceleration(Vec2::new(0.0, GRAVITY)))
                .insert(Velocity::default())
                .insert(Body::default());
        } else {
            sprite.color.set_a(spawning.now / spawning.max);
        }
    }
}

pub fn walker(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Acceleration, &Transform, &Walker)>,
) {
    let player_transform = if let Ok(result) = player_query.get_single() {
        result
    } else {
        return;
    };

    for (mut acceleration, enemy_transform, walker) in enemy_query.iter_mut() {
        acceleration.0.x = (player_transform.translation.x
            - enemy_transform.translation.x)
            .signum()
            * walker.acceleration;
    }
}

pub fn shooter(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Velocity, &mut Shooter, &Transform)>,
) {
    let player_transform = if let Ok(result) = player_query.get_single() {
        result
    } else {
        return;
    };

    for (mut velocity, mut shooter, enemy_transform) in enemy_query.iter_mut() {
        velocity.0.x = (player_transform.translation.x
            - enemy_transform.translation.x)
            .signum()
            * shooter.speed;

        shooter.now += time.delta_seconds();

        while shooter.now >= shooter.max {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                        ..Default::default()
                    },
                    transform: enemy_transform.clone(),
                    texture: asset_server.load("enemy/shooter/bullet.png"),
                    ..Default::default()
                })
                .insert(Flippable)
                .insert(Velocity(Vec2::new(
                    velocity.0.x.signum() * shooter.bullet_speed,
                    0.0,
                )))
                .insert(Cleanup)
                .insert(Bullet);
            shooter.now -= shooter.max;
        }
    }
}

pub fn damage(
    mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
    enemy_query: Query<(&Transform, &Sprite), With<Enemy>>,
) {
    let (mut player, player_transform, player_sprite) =
        if let Ok(result) = player_query.get_single_mut() {
            result
        } else {
            return;
        };

    for (enemy_transform, enemy_sprite) in enemy_query.iter() {
        if collide_aabb::collide(
            player_transform.translation,
            player_sprite.custom_size.unwrap(),
            enemy_transform.translation,
            enemy_sprite.custom_size.unwrap(),
        )
        .is_some()
        {
            player.damage = 1;
        }
    }
}

pub fn bullet(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
) {
    let (mut player, player_transform, player_sprite) =
        if let Ok(result) = player_query.get_single_mut() {
            result
        } else {
            return;
        };

    for (entity, bullet_transform, bullet_sprite) in bullet_query.iter() {
        if collide_aabb::collide(
            player_transform.translation,
            player_sprite.custom_size.unwrap(),
            bullet_transform.translation,
            bullet_sprite.custom_size.unwrap(),
        )
        .is_some()
        {
            player.damage = 1;
            commands.entity(entity).despawn();
        }
    }
}
