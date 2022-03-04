use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverEntity;

pub fn deinit(
    mut commands: Commands,
    query: Query<Entity, With<GameOverEntity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
