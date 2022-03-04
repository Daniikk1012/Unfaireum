use bevy::{prelude::*, app::PluginGroupBuilder};

use game::GamePlugin;
use game_over::GameOverPlugin;
use plugin::MainPlugin;

mod game;
mod game_over;
mod plugin;

pub struct UnfaireumPlugins;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Game,
    GameOver,
}

impl PluginGroup for UnfaireumPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MainPlugin).add(GamePlugin).add(GameOverPlugin);
    }
}
