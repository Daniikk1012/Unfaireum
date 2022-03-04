use bevy::{prelude::*, sprite::collide_aabb};
use rand::Rng;

use crate::plugin::camera::GameCamera;

use super::{
    animation::{Animation, Animations, Flippable, LoadAnimation},
    entity::{GameEntity, GAME_LAYER},
    physics::{Acceleration, Body, Cleanup, Velocity, GRAVITY},
    player::Player,
};

const ENEMY_JUMPER_FALL_ANIMATION: usize = 0;
const ENEMY_JUMPER_JUMP_ANIMATION: usize = 1;
const ENEMY_SIZE: f32 = 128.0;

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
pub struct Jumper {
    impulse: f32,
    speed: f32,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
}

pub fn init(mut commands: Commands) {
    commands.insert_resource(SpawnInterval { now: 0.0, min: 0.5, max: 7.5 });
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
        match rand::thread_rng().gen_range(1..=20) {
            1..=15 => {
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
                    .insert(GameEntity)
                    .insert(Spawning { now: 0.0, max: 1.0 })
                    .insert(Enemy { health: 1 })
                    .insert(Walker { acceleration: 768.0 });
            }
            16..=19 => {
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
                    .insert(GameEntity)
                    .insert(Spawning { now: 0.0, max: 1.5 })
                    .insert(Enemy { health: 2 })
                    .insert(Shooter {
                        speed: 256.0,
                        bullet_speed: 512.0,
                        now: 0.0,
                        max: 2.0,
                    });
            }
            20 => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(
                                ENEMY_SIZE,
                                ENEMY_SIZE * 1.5,
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
                        ..Default::default()
                    })
                    .insert(GameEntity)
                    .insert(Animations {
                        animations: vec![
                            Animation {
                                textures: asset_server
                                    .load_animation("enemy/jumper/fall", 1),
                                ..Default::default()
                            },
                            Animation {
                                textures: asset_server
                                    .load_animation("enemy/jumper/jump", 3),
                                max: 1.0 / 10.0,
                                next: Some(ENEMY_JUMPER_FALL_ANIMATION),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    })
                    .insert(Spawning { now: 0.0, max: 1.0 })
                    .insert(Enemy { health: 3 })
                    .insert(Jumper { impulse: 2560.0, speed: 512.0 });
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
                        custom_size: Some(Vec2::new(64.0, 28.0)),
                        ..Default::default()
                    },
                    transform: enemy_transform.clone(),
                    texture: asset_server.load("enemy/shooter/bullet.png"),
                    ..Default::default()
                })
                .insert(GameEntity)
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

pub fn jumper(
    camera_query: Query<
        (&Transform, &OrthographicProjection),
        With<GameCamera>,
    >,
    mut enemy_query: Query<(
        &mut Velocity,
        &mut Animations,
        &Transform,
        &Body,
        &Jumper,
    )>,
) {
    let (camera_transform, projection) = camera_query.single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    for (mut velocity, mut animations, enemy_transform, body, jumper) in
        enemy_query.iter_mut()
    {
        if velocity.0.x == 0.0 {
            if enemy_transform.translation.x - camera_bounds.left
                < camera_bounds.right - enemy_transform.translation.x
            {
                velocity.0.x = jumper.speed;
            } else {
                velocity.0.x = -jumper.speed;
            }
        }

        if body.bottom {
            velocity.0.y = jumper.impulse;
            animations.current = ENEMY_JUMPER_JUMP_ANIMATION;
            let current = animations.current;
            let mut animation = &mut animations.animations[current];
            animation.frame = 0;
            animation.now = 0.0;
        }
    }
}

pub fn damage(
    mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
    enemy_query: Query<(&Transform, &Sprite), (With<Enemy>, Without<Spawning>)>,
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
