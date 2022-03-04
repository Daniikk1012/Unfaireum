use bevy::prelude::*;

use crate::GameState;

pub struct GameOverPlugin;

mod entity;
mod ui;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(ui::init),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(ui::restart_button),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(entity::deinit),
        );
    }
}
