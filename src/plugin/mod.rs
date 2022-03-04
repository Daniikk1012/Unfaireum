use bevy::prelude::*;

use crate::GameState;

pub struct MainPlugin;

pub mod camera;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Game)
            .add_startup_system(camera::init)
            .add_system_to_stage(CoreStage::PostUpdate, camera::resize);
    }
}
