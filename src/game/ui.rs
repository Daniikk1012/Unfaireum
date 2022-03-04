use bevy::prelude::*;

use super::{
    entity::GameEntity,
    player::{Player, PLAYER_HEALTH_MAX},
};

const HEALTH_SIZE: f32 = 64.0;

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
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(GameEntity)
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
