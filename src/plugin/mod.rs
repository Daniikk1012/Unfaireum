use bevy::prelude::*;

use crate::GameState;

pub const FONT: &'static str = "Montserrat-Regular.ttf";

pub struct MainPlugin;

pub mod camera;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        {
            app.add_system(bevy_web_resizer::web_resize_system);
        }
        app.add_state(GameState::Game)
            .add_startup_system(camera::init)
            .add_system_to_stage(CoreStage::PostUpdate, camera::resize);
    }
}
