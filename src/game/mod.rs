use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};
use physics::TIME_STEP;

use crate::GameState;

mod animation;
mod background;
mod enemy;
mod entity;
mod physics;
mod player;
mod ui;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum GameSystem {
    Acceleration,
    Velocity,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(background::init)
                .with_system(enemy::init)
                .with_system(player::init)
                .with_system(ui::init),
        )
        .add_system_set(build_on_in_stack_update_system_set(
            SystemSet::on_update(GameState::Game).with_system(enemy::prespawn),
        ))
        .add_system_set(build_on_in_stack_update_system_set(
            SystemSet::on_inactive_update(GameState::Game),
        ))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64).chain(
                    |In(input), state: Res<State<GameState>>| {
                        if *state.current() == GameState::Game
                            || state.inactives().contains(&GameState::Game)
                        {
                            input
                        } else {
                            ShouldRun::No
                        }
                    },
                ))
                .with_system(
                    physics::acceleration
                        .label(GameSystem::Acceleration)
                        .before(GameSystem::Velocity),
                )
                .with_system(physics::velocity.label(GameSystem::Velocity))
                .with_system(physics::walls.after(GameSystem::Velocity))
                .with_system(player::bullet.after(GameSystem::Velocity))
                .with_system(enemy::bullet.after(GameSystem::Velocity))
                .with_system(enemy::damage.after(GameSystem::Velocity))
                .with_system(physics::cleanup.after(GameSystem::Velocity)),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(entity::deinit),
        );
    }
}

fn build_on_in_stack_update_system_set(system_set: SystemSet) -> SystemSet {
    system_set
        .with_system(background::resize)
        .with_system(animation::animation)
        .with_system(enemy::spawn)
        .with_system(enemy::walker.before(GameSystem::Acceleration))
        .with_system(enemy::shooter.before(GameSystem::Velocity))
        .with_system(enemy::jumper.before(GameSystem::Velocity))
        .with_system(player::movement.system().before(GameSystem::Velocity))
        .with_system(player::animation.after(GameSystem::Velocity))
        .with_system(animation::flip.after(GameSystem::Velocity))
        .with_system(player::shoot.after(GameSystem::Velocity))
        .with_system(player::damage.after(GameSystem::Velocity))
        .with_system(player::invincibility)
        .with_system(ui::health)
}
