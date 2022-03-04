use bevy::prelude::*;

pub const BACKGROUND_LAYER: f32 = 0.0;
pub const GAME_LAYER: f32 = 1.0;

#[derive(Component)]
pub struct GameEntity;

pub fn deinit(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
