use bevy::{core::FixedTimestep, prelude::*};
use physics::TIME_STEP;

pub struct GamePlugin;

mod camera;
mod physics;
mod player;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum GameSystem {
    Velocity,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera::init)
            .add_startup_system(player::init)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(
                        physics::acceleration.before(GameSystem::Velocity),
                    )
                    .with_system(physics::velocity.label(GameSystem::Velocity))
                    .with_system(physics::walls.after(GameSystem::Velocity))
                    .with_system(physics::cleanup.after(GameSystem::Velocity))
            )
            .add_system(player::movement.before(GameSystem::Velocity))
            .add_system(player::shooting.after(GameSystem::Velocity))
            .add_system_to_stage(CoreStage::PostUpdate, camera::resize);
    }
}
