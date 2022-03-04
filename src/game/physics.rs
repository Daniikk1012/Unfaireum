use bevy::prelude::*;

use crate::plugin::camera::GameCamera;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const GRAVITY: f32 = -4096.0;

#[derive(Default, Component)]
pub struct Acceleration(pub Vec2);

#[derive(Default, Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Cleanup;

#[derive(Default, Component)]
pub struct Body {
    pub left: bool,
    pub bottom: bool,
    pub right: bool,
    pub top: bool,
}

pub fn acceleration(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0 * TIME_STEP;
    }
}

pub fn velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += (velocity.0 * TIME_STEP).extend(0.0);
    }
}

pub fn cleanup(
    mut commands: Commands,
    camera_query: Query<
        (&Transform, &OrthographicProjection),
        With<GameCamera>,
    >,
    cleanup_query: Query<(Entity, &Transform, &Sprite), With<Cleanup>>,
) {
    let (camera_transform, projection) = camera_query.single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    for (entity, transform, sprite) in cleanup_query.iter() {
        let half_size = Vec2::new(
            sprite.custom_size.unwrap().x / 2.0,
            sprite.custom_size.unwrap().y / 2.0,
        );

        if transform.translation.x + half_size.x < camera_bounds.left
            || transform.translation.y + half_size.y < camera_bounds.bottom
            || transform.translation.x - half_size.x > camera_bounds.right
            || transform.translation.y - half_size.y > camera_bounds.top
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn walls(
    mut queries: QuerySet<(
        QueryState<(&Transform, &OrthographicProjection), With<GameCamera>>,
        QueryState<(&mut Transform, &mut Velocity, &mut Body, &Sprite)>,
    )>,
) {
    let (camera_transform, projection) = queries.q0().single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    for (mut transform, mut velocity, mut body, sprite) in
        queries.q1().iter_mut()
    {
        let half_size = Vec2::new(
            sprite.custom_size.unwrap().x / 2.0,
            sprite.custom_size.unwrap().y / 2.0,
        );

        if transform.translation.x - half_size.x < camera_bounds.left {
            transform.translation.x = camera_bounds.left + half_size.x;
            velocity.0.x = 0.0;

            if !body.left {
                body.left = true;
            }

            if body.right {
                body.right = false;
            }
        } else if transform.translation.x + half_size.x > camera_bounds.right {
            transform.translation.x = camera_bounds.right - half_size.y;
            velocity.0.x = 0.0;

            if !body.right {
                body.right = true;
            }

            if body.left {
                body.left = false;
            }
        } else {
            if body.left {
                body.left = false;
            }

            if body.right {
                body.right = false;
            }
        }

        if transform.translation.y - half_size.y < camera_bounds.bottom {
            transform.translation.y = camera_bounds.bottom + half_size.y;
            velocity.0.y = 0.0;

            if !body.bottom {
                body.bottom = true;
            }

            if body.top {
                body.top = false;
            }
        } else if transform.translation.y + half_size.y > camera_bounds.top {
            transform.translation.y = camera_bounds.top - half_size.y;
            velocity.0.y = 0.0;

            if !body.top {
                body.top = true;
            }

            if body.bottom {
                body.bottom = false;
            }
        } else {
            if body.bottom {
                body.bottom = false;
            }

            if body.top {
                body.top = false;
            }
        }
    }
}
