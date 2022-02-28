use bevy::prelude::*;

use crate::camera::GameCamera;

pub const BACKGROUND_LAYER: f32 = 0.0;
pub const GAME_LAYER: f32 = 1.0;

#[derive(Component)]
pub struct Background;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1920.0, 1080.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(960.0, 540.0, BACKGROUND_LAYER),
            texture: asset_server.load("background.png"),
            ..Default::default()
        })
        .insert(Background);
}

pub fn resize(
    mut queries: QuerySet<(
        QueryState<(&Transform, &OrthographicProjection), With<GameCamera>>,
        QueryState<(&mut Transform, &mut Sprite), With<Background>>,
    )>,
) {
    let (camera_transform, projection) = queries.q0().single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    let mut query = queries.q1();
    let (mut transform, mut sprite) = query.single_mut();

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

    if sprite.custom_size.unwrap().x != camera_bounds.right - camera_bounds.left
        || sprite.custom_size.unwrap().y
            != camera_bounds.top - camera_bounds.bottom
    {
        sprite.custom_size = Some(Vec2::new(
            camera_bounds.right - camera_bounds.left,
            camera_bounds.top - camera_bounds.bottom,
        ));
    }
}
