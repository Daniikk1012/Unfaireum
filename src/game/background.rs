use bevy::prelude::*;

use crate::plugin::camera::GameCamera;

use super::entity::{GameEntity, BACKGROUND_LAYER};

#[derive(Component)]
pub struct Background {
    min_size: Vec2,
}

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let min_size = Vec2::new(1920.0, 1080.0);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(min_size.x, min_size.y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(960.0, 540.0, BACKGROUND_LAYER),
            texture: asset_server.load("background.png"),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(Background { min_size });
}

pub fn resize(
    mut queries: QuerySet<(
        QueryState<(&Transform, &OrthographicProjection), With<GameCamera>>,
        QueryState<(&mut Transform, &mut Sprite, &Background)>,
    )>,
) {
    let (camera_transform, projection) = queries.q0().single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    let camera_size = Vec2::new(
        camera_bounds.right - camera_bounds.left,
        camera_bounds.top - camera_bounds.bottom,
    );

    let mut query = queries.q1();
    let (mut transform, mut sprite, background) = query.single_mut();

    if transform.translation.x
        != (camera_bounds.left + camera_bounds.right) / 2.0
        || transform.translation.y
            != (camera_bounds.bottom + camera_bounds.top) / 2.0
    {
        transform.translation.x =
            (camera_bounds.left + camera_bounds.right) / 2.0;
        transform.translation.y =
            (camera_bounds.bottom + camera_bounds.top) / 2.0;
    }

    let size = if camera_size.x * background.min_size.y
        > background.min_size.x * camera_size.y
    {
        Vec2::new(
            camera_size.x,
            background.min_size.y * camera_size.x / background.min_size.x,
        )
    } else {
        Vec2::new(
            background.min_size.x * camera_size.y / background.min_size.y,
            camera_size.y,
        )
    };

    if sprite.custom_size.unwrap().x != size.x
        || sprite.custom_size.unwrap().y != size.y
    {
        sprite.custom_size = Some(size);
    }
}
