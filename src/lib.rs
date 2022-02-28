use bevy::{core::FixedTimestep, prelude::*};
use enemy::SpawnInterval;
use physics::TIME_STEP;

pub struct GamePlugin;

mod animation;
mod background;
mod camera;
mod enemy;
mod physics;
mod player;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum GameSystem {
    Acceleration,
    Velocity,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(background::init)
            .add_startup_system(camera::init)
            .add_startup_system(player::init)
            .init_resource::<SpawnInterval>()
            .add_system(background::resize)
            .add_system(animation::animation)
            .add_system(enemy::prespawn)
            .add_system(enemy::spawn)
            .add_system(enemy::follow.before(GameSystem::Acceleration))
            .add_system(player::movement.system().before(GameSystem::Velocity))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(
                        physics::acceleration
                            .label(GameSystem::Acceleration)
                            .before(GameSystem::Velocity),
                    )
                    .with_system(physics::velocity.label(GameSystem::Velocity))
                    .with_system(physics::walls.after(GameSystem::Velocity))
                    .with_system(player::bullet.after(GameSystem::Velocity))
                    .with_system(enemy::damage.after(GameSystem::Velocity))
                    .with_system(physics::cleanup.after(GameSystem::Velocity)),
            )
            .add_system(player::animation.after(GameSystem::Velocity))
            .add_system(animation::flip.after(GameSystem::Velocity))
            .add_system(player::shooting.after(GameSystem::Velocity))
            .add_system(player::damage.after(GameSystem::Velocity))
            .add_system(player::invincibility)
            .add_system_to_stage(CoreStage::PostUpdate, camera::resize);
    }
}
