use bevy::prelude::*;

use crate::GameState;

use super::entity::GameOverEntity;

const FONT: &'static str = "Montserrat-Regular.ttf";

const BUTTON_UP_COLOR: Color = Color::rgb(0.85, 0.85, 0.85);

#[derive(Component)]
pub struct RestartButton;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(GameOverEntity)
        .with_children(|children| {
            children
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        size: Size::new(Val::Px(700.0), Val::Px(700.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.85, 0.85, 0.85).into(),
                    ..Default::default()
                })
                .with_children(|children| {
                    children.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "GAME OVER".to_string(),
                                style: TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: 128.0,
                                    color: Color::BLACK,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    children
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Px(600.0), Val::Px(300.0)),
                                ..Default::default()
                            },
                            color: BUTTON_UP_COLOR.into(),
                            ..Default::default()
                        })
                        .insert(RestartButton)
                        .with_children(|children| {
                            children.spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Restart".to_string(),
                                        style: TextStyle {
                                            font: asset_server.load(FONT),
                                            font_size: 96.0,
                                            color: Color::BLACK,
                                        },
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                });
        });
}

pub fn restart_button(
    mut state: ResMut<State<GameState>>,
    mut query: Query<
        (&mut UiColor, &Interaction),
        (Changed<Interaction>, With<RestartButton>),
    >,
) {
    let (mut color, interaction) = if let Ok(result) = query.get_single_mut() {
        result
    } else {
        return;
    };

    if *interaction == Interaction::Hovered && color.0 != Color::GRAY {
        color.0 = Color::GRAY;
    } else {
        color.0 = BUTTON_UP_COLOR;
    }

    if *interaction == Interaction::Clicked {
        state.replace(GameState::Game).unwrap();
    }
}
