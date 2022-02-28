use bevy::{prelude::*, sprite::collide_aabb};
use rand::Rng;

use crate::{
    animation::Flippable,
    camera::GameCamera,
    physics::{Acceleration, Body, Velocity, GRAVITY},
    player::Player, background::GAME_LAYER,
};

const ENEMY_SIZE: f32 = 128.0;

pub struct SpawnInterval {
    now: f32,
    max: f32,
}

#[derive(Component)]
pub struct Spawning {
    now: f32,
    max: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
    acceleration: f32,
}

impl Default for SpawnInterval {
    fn default() -> Self {
        SpawnInterval { now: 0.0, max: 7.5 }
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
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
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
            .insert(Spawning { now: 0.0, max: 1.0 });

        spawn_interval.now -= spawn_interval.max;
        spawn_interval.max *= 0.95;
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
                .insert(Body::default())
                .insert(Enemy { health: 1, acceleration: 768.0 });
        } else {
            sprite.color.set_a(spawning.now / spawning.max);
        }
    }
}

pub fn follow(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Acceleration, &Transform, &Enemy)>,
) {
    let player_transform = if let Ok(result) = player_query.get_single() {
        result
    } else {
        return;
    };

    for (mut acceleration, enemy_transform, enemy) in enemy_query.iter_mut() {
        acceleration.0.x = (player_transform.translation.x
            - enemy_transform.translation.x)
            .signum()
            * enemy.acceleration;
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
