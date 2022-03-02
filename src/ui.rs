use bevy::prelude::*;

use crate::{
    camera::UiCamera,
    player::{Player, PLAYER_HEALTH_MAX},
};

const HEALTH_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct RootNode;

#[derive(Component)]
pub struct HealthIndex(u32);

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(RootNode)
        .with_children(|children| {
            for index in 0..PLAYER_HEALTH_MAX {
                children
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(
                                Val::Px(HEALTH_SIZE),
                                Val::Px(HEALTH_SIZE),
                            ),
                            ..Default::default()
                        },
                        image: asset_server.load("bullet.png").into(),
                        ..Default::default()
                    })
                    .insert(HealthIndex(index));
            }
        });
}

pub fn resize(
    camera_query: Query<(&Transform, &OrthographicProjection), With<UiCamera>>,
    mut node_query: Query<&mut Style, With<RootNode>>,
) {
    let (camera_transform, projection) = camera_query.single();

    let camera_bounds = Rect {
        left: camera_transform.translation.x + projection.left,
        bottom: camera_transform.translation.y + projection.bottom,
        right: camera_transform.translation.x + projection.right,
        top: camera_transform.translation.y + projection.top,
    };

    let size = Size::new(
        Val::Px(camera_bounds.right - camera_bounds.left),
        Val::Px(camera_bounds.top - camera_bounds.bottom),
    );

    let mut style = node_query.single_mut();

    if style.size != size {
        style.size = size;
    }
}

pub fn health(
    player_query: Query<&Player>,
    mut health_query: Query<(&mut Visibility, &HealthIndex)>,
) {
    let health = if let Ok(player) = player_query.get_single() {
        player.health
    } else {
        0
    };

    for (mut visibility, HealthIndex(index)) in health_query.iter_mut() {
        let is_visible = *index < health;

        if visibility.is_visible != is_visible {
            visibility.is_visible = is_visible;
        }
    }
}
