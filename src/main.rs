use bevy::prelude::*;
use unfaireum::GamePlugin;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(GamePlugin).run();
}
