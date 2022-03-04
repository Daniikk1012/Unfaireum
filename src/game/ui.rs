use bevy::prelude::*;

use crate::plugin::FONT;

use super::{
    enemy::Score,
    entity::GameEntity,
    player::{Player, PLAYER_HEALTH_MAX},
};

const HEALTH_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct HealthIndex(u32);

#[derive(Component)]
pub struct ScoreText;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(GameEntity)
        .with_children(|children| {
            children
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
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

            children
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Penguins killed: ".to_string(),
                                style: TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                style: TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: 64.0,
                                    color: Color::YELLOW,
                                },
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
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

pub fn score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = query.single_mut();

    text.sections[1].value = score.0.to_string();
}
